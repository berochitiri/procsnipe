@echo off
echo Building procsnipe...
cargo build --release
if %errorlevel% equ 0 (
    echo.
    echo ✓ Build successful!
    echo.
    echo Executable: target\release\procsnipe.exe
    echo.
    echo Run with: .\target\release\procsnipe.exe
) else (
    echo.
    echo ✗ Build failed!
)
pause
