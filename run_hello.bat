
@echo off
echo Setting up Visual Studio Build Tools environment...
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat" x64
cd /d "C:\Users\Administrator\Documents\x-lang"
echo.
echo Building and running examples/hello.x...
cargo run -- run examples\hello.x
