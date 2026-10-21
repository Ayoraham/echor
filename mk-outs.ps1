# mk-outs.ps1
$OutDir = "tests/expected"
New-Item -ItemType Directory -Force -Path $OutDir | Out-Null

# UTF-8 without BOM
$utf8 = New-Object System.Text.UTF8Encoding($false)

function Write-Expected($name, $text) {
    $path = Join-Path (Get-Location) "$OutDir/$name"
    [System.IO.File]::WriteAllText($path, $text, $utf8)
}

Write-Expected "hello1.txt"   "Hello there`n"
Write-Expected "hello2.txt"   "Hello there`n"
Write-Expected "hello1.n.txt" "Hello there"
Write-Expected "hello2.n.txt" "Hello there"