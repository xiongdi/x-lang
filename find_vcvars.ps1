
# 递归搜索 vcvarsall.bat
$searchPaths = @(
    "C:\Program Files\Microsoft Visual Studio",
    "C:\Program Files (x86)\Microsoft Visual Studio",
    "C:\BuildTools",
    "C:\Program Files (x86)\Windows Kits"
)

foreach ($basePath in $searchPaths) {
    if (Test-Path $basePath) {
        Write-Host "Searching in $basePath..."
        $vcvars = Get-ChildItem -Path $basePath -Filter "vcvarsall.bat" -Recurse -ErrorAction SilentlyContinue
        if ($vcvars) {
            Write-Host "Found vcvarsall.bat at:"
            $vcvars.FullName
            exit 0
        }
    }
}

Write-Host "vcvarsall.bat not found. Let's check what's in Program Files/Microsoft Visual Studio:"
if (Test-Path "C:\Program Files\Microsoft Visual Studio") {
    Get-ChildItem "C:\Program Files\Microsoft Visual Studio"
}
