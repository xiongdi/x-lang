
# 直接从 tools/x-cli 运行，不构建 x-codegen-llvm
$vcvars = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"

# 捕获 vcvars64.bat 的环境
$batch = @"
@echo off
call "$vcvars" >nul
set
"@

$batch | Out-File -FilePath "C:\Users\Administrator\Documents\x-lang\temp_env2.bat" -Encoding ASCII
$envOutput = cmd /c "C:\Users\Administrator\Documents\x-lang\temp_env2.bat"

# 解析环境变量
foreach ($line in $envOutput) {
    if ($line -match "^([^=]+)=(.*)$") {
        $name = $matches[1]
        $value = $matches[2]
        Set-Item -Path "env:$name" -Value $value -ErrorAction SilentlyContinue
    }
}

# 从 PATH 中删除 Git 的 bin 目录
$pathParts = $env:PATH -split ';'
$newPathParts = @()
foreach ($part in $pathParts) {
    if ($part -notmatch 'Git\\(usr|mingw64)\\bin') {
        $newPathParts += $part
    }
}
$env:PATH = $newPathParts -join ';'

Write-Host "Environment configured!"
Write-Host "Building from tools/x-cli directory..."

# 直接转到 tools/x-cli 并运行
Set-Location "C:\Users\Administrator\Documents\x-lang\tools\x-cli"
cargo run --no-default-features -- run "..\..\examples\hello.x"

# 清理
Remove-Item "C:\Users\Administrator\Documents\x-lang\temp_env2.bat" -ErrorAction SilentlyContinue
