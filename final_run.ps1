
# 设置环境并运行 x.exe
$vcvars = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"

# 捕获环境
$batch = @"
@echo off
call "$vcvars" >nul
set
"@
$batch | Out-File -FilePath "temp_final_env.bat" -Encoding ASCII
$envOutput = cmd /c "temp_final_env.bat"
foreach ($line in $envOutput) {
    if ($line -match "^([^=]+)=(.*)$") {
        Set-Item -Path "env:$($matches[1])" -Value $matches[2] -ErrorAction SilentlyContinue
    }
}

# 修复 PATH
$pathParts = $env:PATH -split ';'
$newPathParts = @()
foreach ($part in $pathParts) {
    if ($part -notmatch 'Git\\(usr|mingw64)\\bin') {
        $newPathParts += $part
    }
}
$env:PATH = $newPathParts -join ';'

# 运行 x.exe！
Set-Location "C:\Users\Administrator\Documents\x-lang"
Write-Host "Running examples/hello.x..."
& "C:\Users\Administrator\Documents\x-lang\tools\target\debug\x.exe" run "examples\hello.x"

# 清理
Remove-Item "temp_final_env.bat" -ErrorAction SilentlyContinue
