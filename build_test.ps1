
# 简单测试：设置环境并只构建 x-lexer
$vcvars = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"

# 捕获环境变量
$batch = @"
@echo off
call "$vcvars" >nul
set
"@
$batch | Out-File -FilePath "temp_env_test.bat" -Encoding ASCII
$envOutput = cmd /c "temp_env_test.bat"
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

Write-Host "Environment configured. Checking link.exe..."
Get-Command link.exe -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source

Write-Host "`nBuilding x-lexer..."
Set-Location "C:\Users\Administrator\Documents\x-lang\compiler\x-lexer"
cargo build --message-format=short

Write-Host "`nDone!"
Remove-Item "C:\Users\Administrator\Documents\x-lang\temp_env_test.bat" -ErrorAction SilentlyContinue
