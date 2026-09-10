# XYplorer Color Filter Recipe Generator

<p align="center">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-blue.svg">
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows-0078D4.svg">
  <img alt="Built with" src="https://img.shields.io/badge/built%20with-Tauri%202-24C8D8.svg">
  <img alt="Language" src="https://img.shields.io/badge/UI-中文%20%2F%20EN-orange.svg">
</p>

<p align="center"><b>XYplorer 颜色过滤器配方生成器</b> · 开发者 <b>Marcrevo</b> · 基于 XYplorer 官方 Color Filter 语法</p>

---

一款可视化拼装 [XYplorer](https://www.xyplorer.com/) **颜色过滤器（Color Filters）** 规则的小工具：选选择器、设条件、配颜色，实时预览着色效果，一键生成可直接写回 `XYplorer.ini` 的规则文本。

A visual builder for [XYplorer](https://www.xyplorer.com/) **Color Filter** rules — pick a selector, set a condition, choose colors, preview the highlight live, and export the rule text straight into `XYplorer.ini`.

> 💡 演示数据均为通用虚拟文件，不含任何个人信息 · The sample files are generic placeholders with **no personal information**.

---

## ✨ 功能特性 · Features

| | 中文 | English |
|---|---|---|
| 🧩 可视化拼规则 | 12 类选择器 / 运算符 / 单位 / 作用域 / 样式开关 / 配色，所见即所得 | 12 selector families, operators, units, scopes, style switches & palettes — WYSIWYG |
| 👁 实时预览 | 模拟 XYplorer 列表着色，覆盖常见文件类型与状态 | Live list-preview mimicking XYplorer, covering common file types & states |
| 📚 内置配方库 | **98 条**配方，按 6 大类分组折叠浏览（可一键展开/折叠全部） | **98 recipes** in 6 collapsible categories (expand / collapse all) |
| 🎚 多档渐变 | **7 套**渐变模板，一条规则内做程度渐变 | **7 tier-gradient templates** for stepped gradients in one rule |
| 🔗 条件组合器 | AND / OR / NOT 自由组合多条选择器 | Combiner with AND / OR / NOT over multiple selectors |
| 📑 批量生成 | 按扩展名一次给 `.docx/.pdf/.xlsx/…` 批量铺色 | Batch color by extension for whole families |
| 🎨 配色方案 | 多套内置方案（经典 / 色盲友好 Okabe-Ito / 低饱和…） | Built-in palettes (Classic / Colorblind-safe Okabe-Ito / Muted…) |
| 💾 一键读写 | exe 版自动备份 `XYplorer.ini` + 回滚，可反向导入现有规则 | exe build auto-backs-up `XYplorer.ini` with rollback & import |

---

## 🌐 双语界面 · Bilingual UI

界面支持 **中文 / English** 一键切换（右上角 `中 / EN` 按钮），选择会记忆在本地，下次打开沿用。

The UI switches between **中文 / English** with the `中 / EN` button (top-right). The choice is saved in `localStorage` and restored on next launch.

- 配方库、渐变模板、按钮、标签、状态提示均双语呈现。
- 所有 XYplorer 语法示例保持语言无关（规则文本本身不变）。
- Recipe library, tier templates, buttons, labels and status messages are bilingual. XYplorer syntax samples stay language-neutral (the rule text never changes).

---

## 📦 两种形态 · Two builds

| 形态 · Build | 文件 · File | 说明 · Notes |
|---|---|---|
| **exe 版（Tauri）** · exe (Tauri) | `xy-colorfilter-tool.exe` | 可写回本机配置 · writes back to local config. 源码见 `src-tauri/` + `ui/` |
| **浏览器版（单文件 HTML）** · Browser (single-file) | `XYplorer-颜色过滤器配方生成器.html` | 双击即用、零依赖，仅生成规则文本 · zero-dep, generates rule text only |

> 本仓库包含 exe 版源码（`src-tauri/` Rust + `ui/` 前端）。预编译 exe、浏览器版 HTML 与使用说明可在 **[Releases](https://github.com/Marcrevo-bit/xyplorer-colorfilter-tool/releases)** 获取。
> The repo holds the exe source (`src-tauri/` Rust + `ui/` frontend). Prebuilt exe, browser HTML and notes are on **[Releases](https://github.com/Marcrevo-bit/xy-colorfilter-tool/releases)**.

---

## 🚀 快速开始 · Quick start

**exe 版 · exe build**
1. 到 [Releases](https://github.com/Marcrevo-bit/xy-colorfilter-tool/releases) 下载 `XYplorer-ColorFilter-Tool-vX.X.X.exe`；
2. 双击运行（首次需允许 WebView2 运行时）；
3. 在「① 连接 XYplorer」卡片刷新状态，选好规则后「写入 XYplorer」。

**浏览器版 · Browser build**
- 直接双击打开 HTML 文件，在「③ 导出结果」复制规则，手动粘贴到 XYplorer（工具 → 列表管理 → 颜色过滤器 → F6 编辑模式）。

---

## 🧩 配方库与模板 · Library & templates

- **6 大类**：时间热度 · 文件类型 · 属性状态 · 命名场景 · 文件夹 · 高级玩法
- **98 条配方**：时间热度（17）、文件类型（17）、属性状态（16）、命名场景（10）、文件夹（9）、高级玩法（29）
- **7 套渐变模板**：年龄分层 / 文件夹大小分层 / 名称长度分层 / 修改热力分层 等
- 点击任意配方或模板即载入左侧构建器，实时预览后导出。

6 categories: Time & Recency · File Types · Attributes & Size · Naming Patterns · Folders · Advanced. Click any recipe or template to load it into the builder, preview, then export.

---

## 🛠 构建（exe 版）· Build

前置 · Prerequisites: Rust 工具链 + Windows MSVC 构建工具（Visual Studio Build Tools）+ WebView2 运行时 · Rust toolchain + Windows MSVC build tools + WebView2 runtime.

```bash
# Windows（MSVC 工具链，注意让 MSVC 的 link.exe 排在 PortableGit 同名工具之前）
bash build.sh
# 或手动：
cd src-tauri
cargo build --release
```

产物 · Artifact: `src-tauri/target/release/xy-colorfilter-tool.exe`

---

## ✅ 验证 · Verify

```bash
# 前端逻辑自测（需 Node.js）· frontend self-test (Node.js)
node _validate.cjs

# 构建后校验（需 Python + brotli）：输出 SHA256 并确认前端已嵌入
# post-build check (Python + brotli): prints SHA256 and confirms the UI is embedded
python _verify.py
```

---

## 📖 语法速查（精简）· Syntax cheat-sheet

| 片段 · Fragment | 含义 · Meaning |
|---|---|
| `B:` `T:` `L:` | 树+列表 / 仅树 / 仅列表 · tree+list / tree only / list only |
| `ageM: d` | 今天修改 · modified today |
| `*.docx` | 按名称通配 · name wildcard |
| `size: >= 10 MB` | 大小阈值 · size threshold |
| `attr:hidden` | 属性匹配（任一）· attribute (any) |
| `prop:#foldersize: >= 1 GB` | Shell 属性（较慢，建议加预过滤）· shell property (slow, prefilter advised) |
| `A || B || C` | 多档渐变，自上而下首条命中 · multi-tier gradient, top match wins |
| `//m` `//n` | 合并下条文字色 / 仅着色名称列 · merge next text color / name-column only |

> 完整语法见应用内「语法速查」卡片 · Full syntax in the in-app **Syntax** panel.

---

## 📄 许可证 · License

本工具以 **MIT 许可证**开源，版权归 **Marcrevo** 所有。XYplorer 为 XYplorer 官方商标与产品，本工具与其无隶属关系。

Licensed under the **MIT License**, © **Marcrevo**. XYplorer is a trademark of its respective owner; this tool is not affiliated with XYplorer.
