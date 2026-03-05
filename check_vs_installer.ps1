
# 使用 Visual Studio 安装程序查找实例
Write-Host "Looking for VS Installer..."

$installerPath = "C:\Program Files (x86)\Microsoft Visual Studio\Installer\setup.exe"
if (Test-Path $installerPath) {
    Write-Host "VS Installer found at: $installerPath"
}

# 尝试使用 vswhere 查找
$vswherePaths = @(
    "C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe",
    "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
)

foreach ($vswhere in $vswherePaths) {
    if (Test-Path $vswhere) {
        Write-Host "`nFound vswhere at: $vswhere"
        Write-Host "Running vswhere..."
        & $vswhere -all -products * -format json
        break
    }
}
