
# 查找 Visual Studio 并运行 vcvarsall.bat
$possiblePaths = @(
    "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvarsall.bat",
    "C:\Program Files\Microsoft Visual Studio\2022\Professional\VC\Auxiliary\Build\vcvarsall.bat",
    "C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Auxiliary\Build\vcvarsall.bat",
    "C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat",
    "C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvarsall.bat",
    "C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Auxiliary\Build\vcvarsall.bat"
)

$vcvarsPath = $null
foreach ($path in $possiblePaths) {
    if (Test-Path $path) {
        $vcvarsPath = $path
        Write-Host "Found: $vcvarsPath"
        break
    }
}

if (-not $vcvarsPath) {
    Write-Host "Visual C++ Build Tools not found!"
    Write-Host "Please install from: https://visualstudio.microsoft.com/downloads/"
    exit 1
}

# 创建一个临时批处理文件来设置环境并运行 cargo
$batchScript = @"
@echo off
call "$vcvarsPath" x64
cd /d "C:\Users\Administrator\Documents\x-lang"
cargo run -- run examples\hello.x
"@

$batchPath = "C:\Users\Administrator\Documents\x-lang\run_with_vc.bat"
[System.IO.File]::WriteAllText($batchPath, $batchScript)

# 运行批处理文件
& cmd /c $batchPath
