
# 最终重建 - 支持顶级代码（第2轮）
$vcvars = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"

# 捕获环境
$batch = @"
@echo off
call "$vcvars" >nul
set
"@
$batch | Out-File -FilePath "temp_final_env2.bat" -Encoding ASCII
$envOutput = cmd /c "temp_final_env2.bat"
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
Write-Host "Final rebuild with top-level code support (take 2)..."
cargo build --release

Write-Host "`nBuild complete! Now testing examples/hello.x (just print statement, no main function)..."
Set-Location "C:\Users\Administrator\Documents\x-lang"
& "C:\Users\Administrator\Documents\x-lang\tools\target\release\x.exe" run "examples\hello.x"

# 清理
Remove-Item "temp_final_env2.bat" -ErrorAction SilentlyContinue
