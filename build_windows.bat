@echo off
chcp 65001 >nul
echo ==========================================
echo    DeveLlamaGUI Pro - Windows Build Script
echo ==========================================
echo.

REM Check if Rust is installed
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo [ERROR] Rust is not installed!
    echo Please install Rust from: https://rustup.rs/
    echo.
    pause
    exit /b 1
)

echo [INFO] Building DeveLlamaGUI Pro...
echo.

cargo build --release

if %errorlevel% neq 0 (
    echo.
    echo [ERROR] Build failed!
    echo.
    pause
    exit /b 1
)

echo.
echo ==========================================
echo    Build Successful!
echo ==========================================
echo.
echo Executable location:
echo   target\release\DeveLlamaGUI-Pro.exe
echo.
echo You can now run the application by double-clicking:
echo   target\release\DeveLlamaGUI-Pro.exe
echo.
pause
