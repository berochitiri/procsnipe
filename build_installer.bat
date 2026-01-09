@echo off
echo ╔════════════════════════════════════════╗
echo ║   Building procsnipe installer...     ║
echo ╚════════════════════════════════════════╝
echo.

REM Step 1: Build release binary
echo [1/4] Building release binary...
cargo build --release
if %errorlevel% neq 0 (
    echo ✗ Build failed!
    pause
    exit /b 1
)
echo ✓ Build successful
echo.

REM Step 2: Create releases directory
echo [2/4] Creating releases directory...
if not exist "releases" mkdir releases
echo ✓ Directory ready
echo.

REM Step 3: Check for Inno Setup
echo [3/4] Checking for Inno Setup...
set "ISCC=C:\Program Files (x86)\Inno Setup 6\ISCC.exe"
if not exist "%ISCC%" (
    echo ⚠️  Inno Setup not found at: %ISCC%
    echo.
    echo Please install Inno Setup from: https://jrsoftware.org/isdl.php
    echo Or update the path in this script.
    pause
    exit /b 1
)
echo ✓ Inno Setup found
echo.

REM Step 4: Compile installer
echo [4/4] Compiling installer...
"%ISCC%" "installer\setup.iss"
if %errorlevel% neq 0 (
    echo ✗ Installer compilation failed!
    pause
    exit /b 1
)
echo ✓ Installer created!
echo.

echo ╔══════════════════════════════════════╗
echo ║      BUILD COMPLETE! 🎯              ║
echo ╚══════════════════════════════════════╝
echo.
echo Installer location: releases\procsnipe-setup-v1.0.exe
echo Portable version: target\release\procsnipe.exe
echo.
pause
