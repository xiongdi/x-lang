
@echo off
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
echo.
echo PATH after vcvars64:
echo %PATH%
echo.
echo Building and running...
cd /d "C:\Users\Administrator\Documents\x-lang"
cargo run -- run examples\hello.x
