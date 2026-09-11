#!/bin/bash
# ============================================================================
# 构建脚本（Linux / macOS 上的 Git Bash 等 POSIX shell 适用）。
#
# 用途：在本机编译出 Windows 版 .exe（cargo build --release）。
# 关键点：Tauri 在 Windows 上必须用 MSVC 工具链链接，不能用 GNU 工具链。
#   本机装的 MSVC 来自 VS2022(BuildTools 18) + Windows SDK 10.0.26100.0。
#   坑：PortableGit 的 bin 目录里也带一个同名 link.exe（GNU 的硬链接工具），
#       它会和 MSVC 的链接器 link.exe 抢名；若它排在 PATH 前面，链接阶段必炸。
#   解决：先把 MSVC 的 bin 目录 export 到 PATH 最前面，再设好 LIB / INCLUDE，
#   最后切到 src-tauri 目录调用 cargo build --release。
#   与 Windows 原生的 build.bat 等价（build.bat 改用 vcvarsall 加载环境）。
# ============================================================================

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
