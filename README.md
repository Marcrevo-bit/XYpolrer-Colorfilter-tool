# XYplorer 颜色过滤器配方生成器

> 开发者：**Marcrevo** · 基于 XYplorer 官方 Color Filter 语法

一款可视化拼装 [XYplorer](https://www.xyplorer.com/) 颜色过滤器（Color Filters）规则的小工具：选选择器、设条件、配颜色，实时预览着色效果，一键生成可直接写回 `XYplorer.ini` 的规则文本。

## 功能

- **可视化拼规则**：12 类选择器 / 运算符 / 单位 / 作用域 / 样式开关 / 配色，所见即所得
- **实时预览**：模拟 XYplorer 列表着色，覆盖常见文件类型与状态（通用演示数据，不含任何个人信息）
- **约 77 条内置配方**，按 6 大类分组折叠浏览：时间热度 / 文件类型 / 属性状态 / 命名场景 / 文件夹 / 高级玩法
- **条件组合器**：AND / OR / NOT 自由组合多条选择器，拼复杂条件
- **按扩展名批量生成配色**：一次给 `.docx/.pdf/.xlsx/...` 批量铺色
- **3 套配色方案**：经典 / 色盲友好(Okabe-Ito) / 低饱和
- **一键读写本机 `XYplorer.ini`**（exe 版）：自动备份 + 回滚；可反向导入现有规则继续改
- **4 套多档渐变模板**

## 两种形态

| 形态 | 文件 | 说明 |
|---|---|---|
| **exe 版（Tauri）** | `XYplorer配色工具.exe` | 可写回本机配置。源码见 `src-tauri/` + `ui/` |
| **浏览器版（单文件 HTML）** | `XYplorer-颜色过滤器配方生成器.html` | 双击即用、零依赖，仅生成规则文本 |

> 本仓库包含 exe 版源码（`src-tauri/` Rust + `ui/` 前端）。预编译 exe 与浏览器版 HTML、使用说明、速查表等可在 Release 中获取。

## 构建（exe 版）

前置：Rust 工具链 + Windows MSVC 构建工具（Visual Studio Build Tools）+ WebView2 运行时。

```bash
cd src-tauri
cargo build --release
```

产物：`src-tauri/target/release/xy-colorfilter-tool.exe`。

## 验证

- 前端逻辑自测（需 Node.js）：`node _validate.cjs`
- 构建后校验（需 Python + brotli）：`python _verify.py`，会输出 exe 的 SHA256 并解码 Tauri 嵌入的前端资源确认关键 UI 已打包

## 许可证

本工具以 MIT 许可证开源，版权归 **Marcrevo** 所有。XYplorer 为 XYplorer 官方的商标与产品。
