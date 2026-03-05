
# 重新构建并测试
$vcvars = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"

# 捕获环境
$batch = @"
@echo off
call "$vcvars" >nul
set
"@
$batch | Out-File -FilePath "temp_build2_env.bat" -Encoding ASCII
$envOutput = cmd /c "temp_build2_env.bat"
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

# 在发布模式下重新构建
Set-Location "C:\Users\Administrator\Documents\x-lang\tools\x-cli"
Write-Host "Rebuilding x-cli with non-mandatory main function..."
cargo build --release

Write-Host "`nBuild complete! Now testing with examples/hello.x (with main function)..."
Set-Location "C:\Users\Administrator\Documents\x-lang"
& "C:\Users\Administrator\Documents\x-lang\tools\target\release\x.exe" run "examples\hello.x"

# 清理
Remove-Item "temp_build2_env.bat" -ErrorAction SilentlyContinue
