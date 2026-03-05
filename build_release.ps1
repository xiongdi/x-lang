
# 使用发布模式构建以修复栈溢出
$vcvars = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"

# 捕获环境
$batch = @"
@echo off
call "$vcvars" >nul
set
"@
$batch | Out-File -FilePath "temp_release_env.bat" -Encoding ASCII
$envOutput = cmd /c "temp_release_env.bat"
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

# 使用发布模式构建
Set-Location "C:\Users\Administrator\Documents\x-lang\tools\x-cli"
Write-Host "Building x-cli in release mode..."
cargo build --release

Write-Host "`nBuild complete! Let's run hello.x..."
Set-Location "C:\Users\Administrator\Documents\x-lang"
& "C:\Users\Administrator\Documents\x-lang\tools\x-cli\target\release\x.exe" run "examples\hello.x"

# 清理
Remove-Item "..\temp_release_env.bat" -ErrorAction SilentlyContinue
