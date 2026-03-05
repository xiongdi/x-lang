
# 检查已安装的程序
Write-Host "Checking installed Visual Studio products..."

$uninstallPaths = @(
    "HKLM:\Software\Microsoft\Windows\CurrentVersion\Uninstall",
    "HKLM:\Software\Wow6432Node\Microsoft\Windows\CurrentVersion\Uninstall"
)

foreach ($path in $uninstallPaths) {
    if (Test-Path $path) {
        Get-ItemProperty "$path\*" | Where-Object { $_.DisplayName -like "*Visual*" -or $_.DisplayName -like "*Build*" } | Select-Object DisplayName, DisplayVersion, InstallLocation
    }
}

# 检查常见的安装位置
Write-Host "`nChecking common install locations..."
$locations = @(
    "C:\Program Files\Microsoft Visual Studio\2022",
    "C:\Program Files (x86)\Microsoft Visual Studio\2019",
    "C:\Program Files (x86)\Microsoft Visual Studio\2017",
    "C:\BuildTools",
    "D:\BuildTools"
)

foreach ($loc in $locations) {
    if (Test-Path $loc) {
        Write-Host "Found: $loc"
        Get-ChildItem $loc -Recurse -Filter "vcvarsall.bat" -ErrorAction SilentlyContinue | Select-Object -First 3 FullName
    }
}
