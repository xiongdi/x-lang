
# 智能方法：构建单个 crate 并手动运行
$vcvars = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"

# 捕获环境变量
$batch = @"
@echo off
call "$vcvars" >nul
set
"@
$batch | Out-File -FilePath "temp_env3.bat" -Encoding ASCII
$envOutput = cmd /c "temp_env3.bat"
foreach ($line in $envOutput) {
    if ($line -match "^([^=]+)=(.*)$") {
        Set-Item -Path "env:$($matches[1])" -Value $matches[2] -ErrorAction SilentlyContinue
    }
}

# 修复 PATH
$pathParts = $env:PATH -split ';'
$newPathParts = @()
foreach ($part in $pathParts) {
    if ($part -notmatch 'Git\\(usr|mingw64)\\bin') { $newPathParts += $part }
}
$env:PATH = $newPathParts -join ';'

Write-Host "Environment configured!"

# 首先构建不需要 LLVM 的单个 crate
Set-Location "C:\Users\Administrator\Documents\x-lang"

Write-Host "Building x-lexer..."
cargo build -p x-lexer 2>&1 | Select-Object -Last 5

Write-Host "Building x-parser..."
cargo build -p x-parser 2>&1 | Select-Object -Last 5

Write-Host "Building x-typechecker..."
cargo build -p x-typechecker 2>&1 | Select-Object -Last 5

Write-Host "Building x-hir..."
cargo build -p x-hir 2>&1 | Select-Object -Last 5

Write-Host "Building x-perceus..."
cargo build -p x-perceus 2>&1 | Select-Object -Last 5

Write-Host "Building x-interpreter..."
cargo build -p x-interpreter 2>&1 | Select-Object -Last 5

Write-Host "Now building x-cli directly..."
Set-Location "tools\x-cli"
cargo build 2>&1

Write-Host "If build succeeded, run: .\target\debug\x.exe run ..\..\examples\hello.x"

# 清理
Remove-Item "..\temp_env3.bat" -ErrorAction SilentlyContinue
