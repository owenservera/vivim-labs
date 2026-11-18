# One-time copy helper. Explicit -Apply is required. No deletion, staging, or commits.
param(
  [string]$ArborPath = 'C:\0-BlackBoxProject-0\ARBOR',
  [switch]$Apply
)
$ErrorActionPreference = 'Stop'
$here = Split-Path -Parent $MyInvocation.MyCommand.Path
$manifestPath = Join-Path $here 'tracked-files.tsv'
$destinationRoot = Join-Path $here 'files'
if (-not (Test-Path -LiteralPath $ArborPath -PathType Container)) {
  throw "ARBOR folder not found: $ArborPath"
}
$rows = @(Import-Csv -LiteralPath $manifestPath -Delimiter "`t" | Where-Object { $_.decision -eq 'ARCHIVE' })
Write-Host "ARBOR archive candidates: $($rows.Count)"
Write-Host "Source: $ArborPath"
Write-Host "Destination: $destinationRoot"
if (-not $Apply) {
  Write-Host 'DRY RUN. Supply -Apply to copy. No files touched.'
  return
}
$copied = 0
$alreadyPresent = 0
foreach ($row in $rows) {
  $relative = $row.path.Replace('/', [System.IO.Path]::DirectorySeparatorChar)
  $source = Join-Path $ArborPath $relative
  $dest = Join-Path $destinationRoot $relative
  if (-not (Test-Path -LiteralPath $source -PathType Leaf)) {
    throw "Missing file, stopped without deletion: $source"
  }
  # Use Git clean filters so CRLF working trees compare with tracked blobs.
  $actualGitBlob = (& git -C $ArborPath hash-object --path=$($row.path) -- $source)
  if ($LASTEXITCODE -ne 0 -or $actualGitBlob.Trim() -ne $row.git_blob_sha) {
    throw "Changed file vs inventory: $($row.path). Review manually."
  }
  if (Test-Path -LiteralPath $dest -PathType Leaf) {
    if ((Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash -ne (Get-FileHash -LiteralPath $dest -Algorithm SHA256).Hash) {
      throw "Conflicting existing archive copy: $dest"
    }
    $alreadyPresent++
    continue
  }
  $destDir = Split-Path -Parent $dest
  New-Item -ItemType Directory -Path $destDir -Force | Out-Null
  Copy-Item -LiteralPath $source -Destination $dest -ErrorAction Stop
  if ((Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash -ne (Get-FileHash -LiteralPath $dest -Algorithm SHA256).Hash) {
    throw "Byte verification failed: $dest"
  }
  $copied++
}
Write-Host "Verified $copied new copies, $alreadyPresent already identical. ARBOR unchanged."
