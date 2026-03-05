
@echo off
setlocal

:: Set PATH without Git
set PATH=C:\Windows\System32;C:\Windows;C:\Users\Administrator\.cargo\bin

echo Building x...
cd /d C:\Users\Administrator\Documents\x-lang
cargo build

if %ERRORLEVEL% EQU 0 (
    echo.
    echo Build successful! Running hello.x...
    cargo run -- run examples/hello.x
) else (
    echo.
    echo Build failed with error: %ERRORLEVEL%
)

endlocal
