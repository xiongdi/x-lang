
# 从 PATH 中删除 Git 的 usr/bin 和 mingw64/bin 目录
$pathParts = $env:PATH -split ';'
$filteredPath = $pathParts -notmatch 'Git\\usr\\bin' -notmatch 'Git\\mingw64\\bin'
$env:PATH = $filteredPath -join ';'

# 现在运行 cargo
Write-Host "Running with cleaned PATH..."
cargo run -- run examples/hello.x
