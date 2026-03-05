
# 逐步构建所有内容
$vcvars = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"

# 捕获环境变量
$batch = @"
@echo off
call "$vcvars" >nul
set
"@
$batch | Out-File -FilePath "temp_env_all.bat" -Encoding ASCII
$envOutput = cmd /c "temp_env_all.bat"
foreach ($line in $envOutput) {
    if ($line -match "^([^=]+)=(.*)$") {
        Set-Item -Path "env:$($matches[1])" -Value $matches[2] -ErrorAction SilentlyContinue
    }
}

# 修复 PATH - 移除 Git 的 bin 目录
$pathParts = $env:PATH -split ';'
$newPathParts = @()
foreach ($part in $pathParts) {
    if ($part -notmatch 'Git\\(usr|mingw64)\\bin') {
        $newPathParts += $part
    }
}
$env:PATH = $newPathParts -join ';'

Set-Location "C:\Users\Administrator\Documents\x-lang"

Write-Host "Building x-lexer..."
cargo build -p x-lexer

Write-Host "`nBuilding x-parser..."
cargo build -p x-parser

Write-Host "`nBuilding x-typechecker..."
cargo build -p x-typechecker

Write-Host "`nBuilding x-hir..."
cargo build -p x-hir

Write-Host "`nBuilding x-perceus..."
cargo build -p x-perceus

Write-Host "`nBuilding x-codegen (without LLVM)..."
cargo build -p x-codegen

Write-Host "`nBuilding x-interpreter..."
cargo build -p x-interpreter

Write-Host "`nNow building x-cli..."
Set-Location "tools\x-cli"
cargo build

Write-Host "`nAll done! Now let's run hello.x!"
.\target\debug\x.exe run "..\..\examples\hello.x"

# 清理
Remove-Item "..\temp_env_all.bat" -ErrorAction SilentlyContinue
