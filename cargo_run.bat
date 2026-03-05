
@echo off
setlocal

:: Set PATH to only Windows system directories and .cargo\bin
set PATH=C:\Windows\System32;C:\Windows;C:\Users\Administrator\.cargo\bin

echo Building and running examples/hello.x...
echo.

cd /d C:\Users\Administrator\Documents\x-lang
cargo run -- run examples\hello.x

endlocal
