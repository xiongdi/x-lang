
# 使用 PowerShell 捕获 vcvars64.bat 的环境并运行 cargo
$vcvars = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"

# 创建一个临时批处理文件来输出环境变量
$batch = @"
@echo off
call "$vcvars" >nul
set
"@

$batch | Out-File -FilePath "C:\Users\Administrator\Documents\x-lang\temp_env.bat" -Encoding ASCII

# 运行批处理文件并捕获输出
$envOutput = cmd /c "C:\Users\Administrator\Documents\x-lang\temp_env.bat"

# 解析环境变量并设置它们
foreach ($line in $envOutput) {
    if ($line -match "^([^=]+)=(.*)$") {
        $name = $matches[1]
        $value = $matches[2]
        Set-Item -Path "env:$name" -Value $value -ErrorAction SilentlyContinue
    }
}

Write-Host "Environment configured!"

# 从 PATH 中删除 Git 的 usr/bin 和 mingw64/bin 以避免 link.exe 冲突
$pathParts = $env:PATH -split ';'
$newPathParts = @()
foreach ($part in $pathParts) {
    if ($part -notmatch 'Git\\(usr|mingw64)\\bin') {
        $newPathParts += $part
    }
}
$env:PATH = $newPathParts -join ';'

# 验证 link.exe
Write-Host "Checking link.exe..."
$linkCmd = Get-Command link.exe -ErrorAction SilentlyContinue
if ($linkCmd) {
    Write-Host "Found link.exe at: $($linkCmd.Source)"
}

# 现在运行 cargo
Set-Location "C:\Users\Administrator\Documents\x-lang"
Write-Host "`nRunning cargo..."
cargo run -- run examples\hello.x

# 清理临时文件
Remove-Item "C:\Users\Administrator\Documents\x-lang\temp_env.bat" -ErrorAction SilentlyContinue
