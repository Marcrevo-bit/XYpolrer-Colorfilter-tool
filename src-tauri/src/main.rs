// ============================================================================
// XYplorer 颜色过滤器配方生成器 —— Tauri 2 后端（Rust / MSVC）
//
// 职责概述：作为前端 WebView2 与本地 XYplorer 之间的“桥”。
//   - 定位 %APPDATA%\XYplorer\XYplorer.ini，读取 / 写入 color filter 段
//   - 写入前自动备份到 XYplorer\CF_Backups\，支持列表 / 恢复 / 删除 / 清理（保留最近 20 个）
//   - 检测 XYplorer.exe 是否正在运行（运行中写入会被其退出时覆盖，需提醒用户）
//   - 正确处理 XYplorer.ini 的编码：中文 Windows 多为 GBK(ANSI)，用 encoding_rs 探测并按原编码读写，避免中文乱码 / “invalid UTF-8” 报错
//   - 通过 tauri::command 暴露上述能力，前端用 window.__TAURI__.core.invoke 调用
//   时间戳统一按东八区(UTC+8)算，且不引入 chrono，用手写的 civil_from_days 日期换算。
// ============================================================================

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;
use encoding_rs::{Encoding, UTF_8, GBK};

const BACKUP_DIR_NAME: &str = "CF_Backups";

/* ---------------- 路径 ---------------- */

fn xy_dir() -> Result<PathBuf, String> {
    let appdata = std::env::var("APPDATA").map_err(|_| "无法读取环境变量 %APPDATA%".to_string())?;
    Ok(PathBuf::from(appdata).join("XYplorer"))
}

fn ini_path() -> Result<PathBuf, String> {
    Ok(xy_dir()?.join("XYplorer.ini"))
}

fn backup_dir() -> Result<PathBuf, String> {
    Ok(xy_dir()?.join(BACKUP_DIR_NAME))
}

/* ---------------- 时间戳（本地 UTC+8，不引入 chrono） ---------------- */

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn local_stamp() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0) as i64
        + 8 * 3600; // 东八区
    let days = secs / 86_400;
    let rem = secs % 86_400;
    let (y, m, d) = civil_from_days(days);
    format!(
        "{:04}{:02}{:02}-{:02}{:02}{:02}",
        y,
        m,
        d,
        rem / 3600,
        (rem % 3600) / 60,
        rem % 60
    )
}

/* ---------------- 数据结构 ---------------- */

#[derive(Serialize)]
pub struct IniStatus {
    path: String,
    exists: bool,
    size: u64,
    modified: String,
    backup_dir: String,
    backups: usize,
    xy_running: bool,
    xy_exe: String,
}

#[derive(Serialize)]
pub struct BackupInfo {
    name: String,
    size: u64,
    modified: String,
}

/* ---------------- 编码处理（XYplorer.ini 在中文 Windows 上多为 GBK/ANSI） ---------------- */

/// 探测 ini 文件编码：BOM → UTF-8；能合法解析为 UTF-8 → UTF-8；否则按 GBK（中文 Windows 默认 ANSI）
fn detect_enc(bytes: &[u8]) -> (&'static Encoding, bool) {
    let had_bom = bytes.starts_with(&[0xEF, 0xBB, 0xBF]);
    if had_bom {
        (UTF_8, true)
    } else if std::str::from_utf8(bytes).is_ok() {
        (UTF_8, false)
    } else {
        (GBK, false)
    }
}

/// 读取 ini 文本，按探测到的编码解码为 Rust String（UTF-8）。GBK 字节不再报 “stream did not contain valid UTF-8”
fn read_ini_text(path: &PathBuf) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|e| format!("读取失败：{}", e))?;
    let (enc, had_bom) = detect_enc(&bytes);
    let (decoded, _, _) = enc.decode(&bytes);
    let mut text = decoded.into_owned();
    if had_bom && text.starts_with('\u{feff}') {
        text.remove(0);
    }
    Ok(text)
}

/// 把文本按文件原有编码写回（保留 BOM），保证中文不乱码
fn write_ini_text(path: &PathBuf, content: &str) -> Result<(), String> {
    let existing = fs::read(path).unwrap_or_default();
    let (enc, had_bom) = detect_enc(&existing);
    let (encoded, _, _) = enc.encode(content);
    let mut out = encoded.into_owned();
    if had_bom {
        let mut with_bom = Vec::with_capacity(out.len() + 3);
        with_bom.extend_from_slice(&[0xEF, 0xBB, 0xBF]);
        with_bom.extend_from_slice(&out);
        out = with_bom;
    }
    fs::write(path, &out).map_err(|e| format!("写入失败：{}", e))
}

/* ---------------- 命令 ---------------- */

/// 返回 XYplorer.ini 的完整路径与状态
#[tauri::command]
fn get_ini_status() -> Result<IniStatus, String> {
    let p = ini_path()?;
    let exists = p.exists();
    let (size, modified) = match fs::metadata(&p) {
        Ok(md) => (
            md.len(),
            md.modified()
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| {
                    let s = d.as_secs() as i64 + 8 * 3600;
                    let (y, mo, dd) = civil_from_days(s / 86_400);
                    let rem = s % 86_400;
                    format!(
                        "{:04}-{:02}-{:02} {:02}:{:02}",
                        y,
                        mo,
                        dd,
                        rem / 3600,
                        (rem % 3600) / 60
                    )
                })
                .unwrap_or_else(|| "未知".to_string()),
        ),
        Err(_) => (0, "-".to_string()),
    };
    let bd = backup_dir()?;
    let backups = fs::read_dir(&bd)
        .map(|rd| rd.filter_map(|e| e.ok()).count())
        .unwrap_or(0);

    Ok(IniStatus {
        path: p.to_string_lossy().to_string(),
        exists,
        size,
        modified,
        backup_dir: bd.to_string_lossy().to_string(),
        backups,
        xy_running: is_xyplorer_running(),
        xy_exe: find_xyplorer().unwrap_or_default(),
    })
}

/// 读取 ini 全文
#[tauri::command]
fn read_ini() -> Result<String, String> {
    let p = ini_path()?;
    if !p.exists() {
        return Err(format!("配置文件不存在：{}", p.display()));
    }
    read_ini_text(&p)
}

/// 写入 ini（先自动备份）。返回备份文件名
#[tauri::command]
fn write_ini(content: String) -> Result<String, String> {
    let p = ini_path()?;
    if !p.exists() {
        return Err(format!("配置文件不存在，拒绝写入：{}", p.display()));
    }
    if content.trim().is_empty() {
        return Err("内容为空，已取消写入".to_string());
    }
    // 基础校验：必须包含 [ColorFilter] 段，防止写坏整个配置
    if !content.contains("[ColorFilter]") {
        return Err("内容中缺少 [ColorFilter] 段，已取消写入（避免损坏配置）".to_string());
    }
    let bak = create_backup()?;
    write_ini_text(&p, &content)?;
    Ok(bak)
}

/// 创建备份，返回备份文件名
#[tauri::command]
fn create_backup() -> Result<String, String> {
    let src = ini_path()?;
    if !src.exists() {
        return Err("配置文件不存在，无法备份".to_string());
    }
    let bd = backup_dir()?;
    fs::create_dir_all(&bd).map_err(|e| format!("无法创建备份目录：{}", e))?;
    let name = format!("XYplorer.ini.bak-{}", local_stamp());
    fs::copy(&src, bd.join(&name)).map_err(|e| format!("备份失败：{}", e))?;
    prune_backups()?;
    Ok(name)
}

/// 列出所有备份（按时间倒序）
#[tauri::command]
fn list_backups() -> Result<Vec<BackupInfo>, String> {
    let bd = backup_dir()?;
    if !bd.exists() {
        return Ok(vec![]);
    }
    let mut out: Vec<BackupInfo> = fs::read_dir(&bd)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name()
                .to_string_lossy()
                .starts_with("XYplorer.ini.bak-")
        })
        .map(|e| {
            let md = e.metadata().ok();
            let size = md.as_ref().map(|m| m.len()).unwrap_or(0);
            let modified = md
                .as_ref()
                .and_then(|m| m.modified().ok())
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| {
                    let s = d.as_secs() as i64 + 8 * 3600;
                    let (y, mo, dd) = civil_from_days(s / 86_400);
                    let rem = s % 86_400;
                    format!(
                        "{:04}-{:02}-{:02} {:02}:{:02}",
                        y,
                        mo,
                        dd,
                        rem / 3600,
                        (rem % 3600) / 60
                    )
                })
                .unwrap_or_else(|| "未知".to_string());
            BackupInfo {
                name: e.file_name().to_string_lossy().to_string(),
                size,
                modified,
            }
        })
        .collect();
    out.sort_by(|a, b| b.name.cmp(&a.name));
    Ok(out)
}

/// 恢复指定备份（恢复前会先备份当前）
#[tauri::command]
fn restore_backup(name: String) -> Result<String, String> {
    if !name.starts_with("XYplorer.ini.bak-") {
        return Err("非法的备份文件名".to_string());
    }
    let src = backup_dir()?.join(&name);
    if !src.exists() {
        return Err("备份文件不存在".to_string());
    }
    let pre = create_backup()?;
    fs::copy(&src, ini_path()?).map_err(|e| format!("恢复失败：{}", e))?;
    Ok(pre)
}

/// 删除指定备份
#[tauri::command]
fn delete_backup(name: String) -> Result<(), String> {
    if !name.starts_with("XYplorer.ini.bak-") {
        return Err("非法的备份文件名".to_string());
    }
    fs::remove_file(backup_dir()?.join(&name)).map_err(|e| format!("删除失败：{}", e))
}

/// 保留最近 20 个备份
fn prune_backups() -> Result<(), String> {
    let mut list = list_backups()?;
    if list.len() <= 20 {
        return Ok(());
    }
    list.sort_by(|a, b| b.name.cmp(&a.name));
    for item in list.into_iter().skip(20) {
        let _ = fs::remove_file(backup_dir()?.join(&item.name));
    }
    Ok(())
}

/// 检测 XYplorer 是否正在运行（运行中写入会被退出时覆盖）
#[tauri::command]
fn is_xyplorer_running() -> bool {
    let out = Command::new("tasklist")
        .args(["/FI", "IMAGENAME eq XYplorer.exe", "/NH"])
        .output();
    match out {
        Ok(o) => {
            let s = String::from_utf8_lossy(&o.stdout).to_lowercase();
            s.contains("xyplorer.exe")
        }
        Err(_) => false,
    }
}

/// 在资源管理器中打开备份目录
#[tauri::command]
fn open_backup_folder() -> Result<(), String> {
    let bd = backup_dir()?;
    fs::create_dir_all(&bd).map_err(|e| e.to_string())?;
    Command::new("explorer")
        .arg(&bd)
        .spawn()
        .map_err(|e| format!("打开失败：{}", e))?;
    Ok(())
}

/// 定位 XYplorer 主程序（用于提示先退出程序）
fn find_xyplorer() -> Option<String> {
    let mut cands: Vec<PathBuf> = Vec::new();
    for root in ["C:\\", "D:\\", "E:\\"] {
        cands.push(PathBuf::from(format!("{}Program Files\\XYplorer\\XYplorer.exe", root)));
        cands.push(PathBuf::from(format!(
            "{}Program Files (x86)\\XYplorer\\XYplorer.exe",
            root
        )));
        cands.push(PathBuf::from(format!("{}Software\\XYplorer\\XYplorer.exe", root)));
        cands.push(PathBuf::from(format!("{}XYplorer\\XYplorer.exe", root)));
    }
    cands.into_iter().find(|p| p.exists())
        .map(|p| p.to_string_lossy().to_string())
}

/// 把 XYplorer 的 [ColorFilter] 段替换为给定内容，其余段原样保留
#[tauri::command]
fn replace_colorfilter_section(section: String) -> Result<String, String> {
    let p = ini_path()?;
    if !p.exists() {
        return Err("配置文件不存在".to_string());
    }
    let raw = read_ini_text(&p)?;
    let new_raw = splice_section(&raw, "[ColorFilter]", &section);
    if new_raw == raw {
        return Err("未找到 [ColorFilter] 段，或内容无变化".to_string());
    }
    let bak = create_backup()?;
    write_ini_text(&p, &new_raw)?;
    Ok(bak)
}

/// 提取某个 ini 段的原始文本（含段标题行，直到下一个 [段] 或文件末尾）
#[tauri::command]
fn extract_section(section_name: String) -> Result<String, String> {
    let raw = read_ini_text(&ini_path()?)?;
    let start = raw
        .find(&section_name)
        .ok_or_else(|| format!("未找到段：{}", section_name))?;
    let rest = &raw[start..];
    let end = rest[section_name.len()..]
        .find("\n[")
        .map(|i| i + section_name.len() + 1)
        .unwrap_or(rest.len());
    Ok(rest[..end].to_string())
}

/// 通用：替换/插入段
fn splice_section(raw: &str, header: &str, body: &str) -> String {
    if let Some(start) = raw.find(header) {
        let rest = &raw[start..];
        let end_rel = rest[header.len()..].find("\n[");
        let end_abs = match end_rel {
            Some(i) => start + header.len() + i + 1,
            None => raw.len(),
        };
        let mut out = String::with_capacity(raw.len() + body.len());
        out.push_str(&raw[..start]);
        out.push_str(body.trim_end());
        out.push('\n');
        if end_abs < raw.len() {
            out.push('\n');
            out.push_str(&raw[end_abs..]);
        } else {
            out.push('\n');
        }
        out
    } else {
        // 段不存在则追加到末尾
        let mut out = String::from(raw.trim_end());
        out.push_str("\n\n");
        out.push_str(body.trim_end());
        out.push('\n');
        out
    }
}

/* ---------------- 入口 ---------------- */

// 应用入口：注册全部 Tauri 命令，加载前端（ui/index.html 或 dev URL），启动事件循环。
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn gbk_ini_roundtrip() {
        // 模拟中文 Windows 上 GBK 编码的 XYplorer.ini：含中文标题，绝不能是 UTF-8
        let (enc, _, _) = GBK.encode("名称=红色配置\nCount=1");
        let gbk_bytes = enc.into_owned();
        assert!(std::str::from_utf8(&gbk_bytes).is_err(), "测试样本必须是非法 UTF-8 的 GBK 字节");

        let dir = std::env::temp_dir().join("xy_cf_test_gbk");
        let _ = std::fs::create_dir_all(&dir);
        let p = dir.join("XYplorer.ini");
        {
            let mut f = std::fs::File::create(&p).unwrap();
            f.write_all(&gbk_bytes).unwrap();
        }

        // 旧实现 fs::read_to_string 会报 "stream did not contain valid UTF-8"，新实现应成功解码中文
        let text = read_ini_text(&p).expect("read_ini_text 在 GBK 文件上应成功");
        assert!(text.contains("红色配置"), "中文应被正确解码，实际：{}", text);

        // 写回应保留 GBK 编码，中文不乱码
        let new_content = format!("{}\n新增=蓝色规则", text);
        write_ini_text(&p, &new_content).expect("write_ini_text 应成功");
        let bytes_back = std::fs::read(&p).unwrap();
        let (dec, _, _) = GBK.decode(&bytes_back);
        let back = dec.into_owned();
        assert!(back.contains("蓝色规则"), "写回后中文应仍为合法 GBK：{}", back);
        // 不应混入 UTF-8 里的中文字节（用 UTF-8 解析应失败，证明是 GBK）
        assert!(std::str::from_utf8(&bytes_back).is_err(), "写回产物必须仍是 GBK 而非 UTF-8");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn utf8_with_bom_preserved() {
        let dir = std::env::temp_dir().join("xy_cf_test_bom");
        let _ = std::fs::create_dir_all(&dir);
        let p = dir.join("XYplorer.ini");
        let content = "Name=Config\nCount=0";
        std::fs::write(&p, [&[0xEFu8, 0xBB, 0xBF], content.as_bytes()].concat()).unwrap();

        let text = read_ini_text(&p).unwrap();
        assert_eq!(text, content, "BOM 应在读取时被剥离");

        write_ini_text(&p, "Name=Config\nCount=2").unwrap();
        let bytes = std::fs::read(&p).unwrap();
        assert_eq!(&bytes[..3], &[0xEF, 0xBB, 0xBF], "写回应保留 UTF-8 BOM");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn real_ini_extract_section_no_utf8_error() {
        // 直接调用真实的 Tauri 命令入口，验证「读取现有规则」在真实 XYplorer.ini 上不再报 UTF-8 错误
        let p = match ini_path() {
            Ok(p) if p.exists() => p,
            _ => {
                eprintln!("跳过：本机未找到 XYplorer.ini（CI/无 XYplorer 环境）");
                return;
            }
        };
        let raw = match read_ini_text(&p) {
            Ok(t) => t,
            Err(e) => panic!("真实 ini 读取失败（原 bug 复现）：{}", e),
        };
        // 进一步验证 extract_section 入口本身可用
        let sec = extract_section("[ColorFilter]".to_string())
            .expect("extract_section 在真实 ini 上应成功，不应报 stream did not contain valid UTF-8");
        assert!(sec.contains("[ColorFilter]"), "提取结果应包含 [ColorFilter] 段头");
        assert!(raw.len() > 0, "ini 内容不应为空");
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_ini_status,
            read_ini,
            write_ini,
            create_backup,
            list_backups,
            restore_backup,
            delete_backup,
            is_xyplorer_running,
            open_backup_folder,
            replace_colorfilter_section,
            extract_section
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
