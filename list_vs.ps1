
$vsPath = "C:\Program Files\Microsoft Visual Studio"
if (Test-Path $vsPath) {
    Write-Host "Contents of $vsPath :"
    Get-ChildItem $vsPath | ForEach-Object { Write-Host "  $($_.Name)" }

    # Check each subdirectory
    $subDirs = Get-ChildItem $vsPath -Directory
    foreach ($subDir in $subDirs) {
        Write-Host "`nContents of $($subDir.FullName) :"
        Get-ChildItem $subDir.FullName -ErrorAction SilentlyContinue | ForEach-Object { Write-Host "  $($_.Name)" }
    }
} else {
    Write-Host "$vsPath does not exist"
}

# Also check for BuildTools in other locations
Write-Host "`nChecking C:\BuildTools..."
if (Test-Path "C:\BuildTools") {
    Get-ChildItem "C:\BuildTools" -Recurse -Filter "vcvarsall.bat" -ErrorAction SilentlyContinue
}

Write-Host "`nChecking C:\Program Files (x86)\Windows Kits..."
if (Test-Path "C:\Program Files (x86)\Windows Kits") {
    Get-ChildItem "C:\Program Files (x86)\Windows Kits"
}
