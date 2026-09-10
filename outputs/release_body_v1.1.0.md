## 更新内容 · What's new (v1.1.0)

- 🌐 **新增中 / EN 双语界面**：右上角 `中 / EN` 一键切换，选择记忆在本地 · Added a **中文 / English** toggle (top-right); choice is saved locally.
- 📚 **内置配方库扩容至 98 条**：按 6 大类（时间热度 / 文件类型 / 属性状态 / 命名场景 / 文件夹 / 高级玩法）分组折叠 · Recipe library expanded to **98 recipes** in 6 categories.
- 🎚 **多档渐变模板独立成卡**：共 7 套，点击即载入 · Tier-gradient templates moved to their own card (7 templates), click to load.
- 🐛 **修复内容展示不全**：配方库与模板区改为可滚动，并新增「展开全部 / 折叠全部」· Fixed truncated display — both areas are now scrollable with expand/collapse-all.
- 📖 **README 中英双语 + 美化**：徽章、表格、速查表 · Bilingual, beautified README with badges, tables and a cheat-sheet.
- 🔧 版本升级至 **v1.1.0** · Bumped to **v1.1.0**.

## 下载与校验 · Download & verify

1. 下载 `XYplorer-ColorFilter-Tool-v1.1.0.exe`
2. 校验 SHA256（完整值见随附 `.SHA256.txt`）：
   `b20a5c040008b1cabd6749e586d9fecdefba204c7bee3b1ace270aa37e442440`
3. PowerShell 校验命令 · verify in PowerShell:
   ```powershell
   (Get-FileHash .\XYplorer-ColorFilter-Tool-v1.1.0.exe -Algorithm SHA256).Hash
   ```

## 使用 · Usage

- **exe 版**：双击运行 → 「① 连接 XYplorer」刷新状态 → 选规则 →「写入 XYplorer」（自动备份 + 可回滚）。
- **浏览器版**：双击 HTML →「③ 导出结果」复制规则，手动粘贴到 XYplorer（工具 → 列表管理 → 颜色过滤器 → F6 编辑模式）。

## 许可证 · License

MIT · © Marcrevo. XYplorer 为 XYplorer 官方商标与产品，本工具与其无隶属关系。
