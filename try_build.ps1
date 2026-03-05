
# 重新排序 PATH，使 Git 的 bin 目录移到末尾
$pathParts = $env:PATH -split ';'
$gitPaths = @()
$otherPaths = @()

foreach ($part in $pathParts) {
    if ($part -match 'Git\\(usr|mingw64)\\bin') {
        $gitPaths += $part
    } elseif ($part -ne '') {
        $otherPaths += $part
    }
}

$newPath = ($otherPaths + $gitPaths) -join ';'
$env:PATH = $newPath

Write-Host "PATH reordered. Git paths moved to end."
Write-Host ""

# 检查 link.exe
Write-Host "Checking for link.exe..."
$linkPath = Get-Command link.exe -ErrorAction SilentlyContinue
if ($linkPath) {
    Write-Host "Found link.exe at: $($linkPath.Source)"
    & link.exe /? 2>&1 | Select-Object -First 3
} else {
    Write-Host "link.exe not found in PATH"
}

Write-Host ""
Write-Host "Attempting to build..."

# 尝试构建
Set-Location "C:\Users\Administrator\Documents\x-lang"
cargo build --message-format=short 2>&1 | Select-Object -Last 50
