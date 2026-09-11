@echo off
REM ============================================================================
REM 构建脚本（Windows 原生，双击或 cmd 运行；等价于 Linux 上的 build.sh）。
REM
REM 用途：在本机编译出 Windows 版 .exe（cargo build --release）。
REM 关键点：Tauri 在 Windows 上必须用 MSVC 工具链（cl/link），不能用 GNU。
REM   本脚本先通过 VS2022 BuildTools 的 vcvarsall.bat 把 MSVC 与 Windows SDK
REM   的编译/链接/头文件/库路径一次性注入当前环境，再切到 src-tauri 跑 cargo。
REM   相比 build.sh 自己拼 PATH/LIB/INCLUDE，这里直接复用 vcvarsall 更稳。
REM ============================================================================
call "C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvarsall.bat" x64
cd /d "%~dp0src-tauri"
cargo build --release
echo.
echo === 构建结束，退出码 %ERRORLEVEL% ===
if exist "target\release\xy-colorfilter-tool.exe" (
  echo 产物: %~dp0src-tauri\target\release\xy-colorfilter-tool.exe
)
pause
