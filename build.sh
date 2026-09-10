#!/bin/bash
# 构建 XYplorer 配色工具（release）
# 说明：Windows 上 Tauri 必须用 MSVC 工具链；本机装的是 VS18 BuildTools + Win SDK 10.0.26100.0。
#       注意 PATH 中 PortableGit 自带一个同名 link.exe（GNU 的硬链接工具），
#       必须让 MSVC 的 bin 排在最前，否则链接阶段会失败。

MSVC='/c/Program Files (x86)/Microsoft Visual Studio/18/BuildTools/VC/Tools/MSVC/14.50.35717'
SDK='/c/Program Files (x86)/Windows Kits/10'
SDKV='10.0.26100.0'

export PATH="$MSVC/bin/Hostx64/x64:$PATH"
export LIB="C:\\Program Files (x86)\\Microsoft Visual Studio\\18\\BuildTools\\VC\\Tools\\MSVC\\14.50.35717\\lib\\x64;C:\\Program Files (x86)\\Windows Kits\\10\\Lib\\$SDKV\\um\\x64;C:\\Program Files (x86)\\Windows Kits\\10\\Lib\\$SDKV\\ucrt\\x64"
export INCLUDE="C:\\Program Files (x86)\\Microsoft Visual Studio\\18\\BuildTools\\VC\\Tools\\MSVC\\14.50.35717\\include;C:\\Program Files (x86)\\Windows Kits\\10\\Include\\$SDKV\\um;C:\\Program Files (x86)\\Windows Kits\\10\\Include\\$SDKV\\ucrt;C:\\Program Files (x86)\\Windows Kits\\10\\Include\\$SDKV\\shared"

echo "=== 校验工具链 ==="
echo "link.exe -> $(which link)"
echo "cl.exe   -> $(which cl)"

cd "$(dirname "$0")/src-tauri" || exit 1
cargo build --release 2>&1
echo "=== 退出码 $? ==="
