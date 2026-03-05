
# 太好了！找到了 VS BuildTools 已安装！现在让我们查找 vcvarsall.bat！
$vsPath = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools"

Write-Host "Checking in: $vsPath\VC\Auxiliary\Build

$vcvarsPath = "$vsPath\VC\Auxiliary\Build\vcvarsall.bat"
if (Test-Path $vcvarsPath) {
    Write-Host "Found vcvarsall.bat at $vcvarsPath"

    # 创建一个批处理文件来设置环境并运行 cargo
    $batchContent = @"
@echo off
call "$vcvarsPath" x64
cd /d "C:\Users\Administrator\Documents\x-lang"
echo Environment configured! Now building...
cargo run -- run examples\hello.x
"@

    $batchFile = "C:\Users\Administrator\Documents\x-lang\run_hello_final.bat"
    [System.IO.File]::WriteAllText($batchFile, $batchContent)

    Write-Host "Running batch file..."
    & cmd /c $batchFile
} else {
    Write-Host "vcvarsall.bat not found at $vcvarsPath"
    Write-Host "Let's check what's in $vsPath\VC\Auxiliary\Build"
    if (Test-Path "$vsPath\VC\Auxiliary\Build") {
        Get-ChildItem "$vsPath\VC\Auxiliary\Build"
    } else {
        Write-Host "Directory doesn't exist"
        Write-Host "Let's check if we have the VC workload installed..."

        # Let's check what's installed
        $installer = "C:\Program Files (x86)\Microsoft Visual Studio\Installer\setup.exe"
        Write-Host "Launching VS Installer to install VC workload..."
        Write-Host "Please install the 'Desktop development with C++' workload!"
    }
}
