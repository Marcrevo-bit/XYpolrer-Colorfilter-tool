@echo off
REM 构建 XYplorer 配色工具（release）
REM 必须在 MSVC 环境下构建；此脚本先加载 vcvarsall，再调用 cargo
call "C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvarsall.bat" x64
cd /d "%~dp0src-tauri"
cargo build --release
echo.
echo === 构建结束，退出码 %ERRORLEVEL% ===
if exist "target\release\xy-colorfilter-tool.exe" (
  echo 产物: %~dp0src-tauri\target\release\xy-colorfilter-tool.exe
)
pause
