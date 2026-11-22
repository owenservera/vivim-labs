<#
.SYNOPSIS
Private incremental archive of Desktop Commander's local argument-only logs.
.DESCRIPTION
Captures bytes from the active claude_tool_call.log and static rotated logs.
Never parses, redacts, uploads, prints, or commits raw arguments. Keep local only.
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)] [string]$SourceDirectory,
    [string]$ArchiveRoot = (Join-Path (Split-Path -Parent $PSScriptRoot) 'local-logs')
)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
if (-not (Test-Path -LiteralPath $SourceDirectory -PathType Container)) { throw "SOURCE_DIR_NOT_FOUND: $SourceDirectory" }
$srcDir = [IO.Path]::GetFullPath($SourceDirectory)
$archive = [IO.Path]::GetFullPath($ArchiveRoot)
if ($archive.StartsWith($srcDir + [IO.Path]::DirectorySeparatorChar, [StringComparison]::OrdinalIgnoreCase)) { throw 'Destination may not be inside the rotating source directory.' }
function Ensure-Dir([string]$path) { [void][IO.Directory]::CreateDirectory($path) }
function Write-Json([string]$path, [object]$obj) {
    Ensure-Dir (Split-Path -Parent $path)
    [IO.File]::WriteAllText($path, ($obj | ConvertTo-Json -Depth 6), [Text.UTF8Encoding]::new($false))
}
$now = (Get-Date).ToUniversalTime()
$day = $now.ToString('yyyy-MM-dd')
$root = Join-Path $archive 'raw/arguments'
$rotated = Join-Path $root 'rotated'
$segments = Join-Path $root ('segments/' + $day)
$stateDir = Join-Path $archive 'state'
$receiptDir = Join-Path $archive 'receipts'
Ensure-Dir $rotated; Ensure-Dir $segments; Ensure-Dir $stateDir; Ensure-Dir $receiptDir
$statePath = Join-Path $stateDir 'argument-active.json'
$state = $null
if (Test-Path -LiteralPath $statePath) {
    try { $state = (Get-Content -LiteralPath $statePath -Raw | ConvertFrom-Json) } catch { throw 'Checkpoint file invalid: refusing unsafe delta offset; preserve and inspect state.' }
}
$seenRotated = Join-Path $stateDir 'rotated-sha256.txt'
$seen = [Collections.Generic.HashSet[string]]::new([StringComparer]::OrdinalIgnoreCase)
if (Test-Path -LiteralPath $seenRotated) {
    foreach ($hash in [IO.File]::ReadLines($seenRotated)) { if ($hash.Length -eq 64) { [void]$seen.Add($hash) } }
}
$rotatedNew = 0; $rotatedBytes = [long]0
$candidates = @(Get-ChildItem -LiteralPath $srcDir -File -Filter 'claude_tool_call*' -ErrorAction Stop)
foreach ($file in $candidates) {
    if ($file.Name -eq 'claude_tool_call.log') { continue }
    # A rotated file is immutable after rename; copy once by content hash.
    $sha = (Get-FileHash -LiteralPath $file.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($seen.Contains($sha)) { continue }
    $dest = Join-Path $rotated ($sha + '-' + $file.Name)
    if (-not (Test-Path -LiteralPath $dest)) { [IO.File]::Copy($file.FullName, $dest) }
    $destSha = (Get-FileHash -LiteralPath $dest -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($sha -ne $destSha) { throw 'ROTATED_HASH_MISMATCH: destination content differs from source' }
    [IO.File]::AppendAllText($seenRotated, $sha + [Environment]::NewLine, [Text.UTF8Encoding]::new($false))
    [void]$seen.Add($sha)
    $rotatedNew++; $rotatedBytes += $file.Length
}

$active = Join-Path $srcDir 'claude_tool_call.log'
$deltaBytes = [long]0; $segmentPath = $null; $offset = [long]0; $rollover = $false
if (Test-Path -LiteralPath $active -PathType Leaf) {
    $fileInfo = Get-Item -LiteralPath $active
    # Creation time identifies the active file across a size-preserving rotation.
    $creationUtc = $fileInfo.CreationTimeUtc.ToString('o')
    # ConvertFrom-Json can materialize ISO timestamps as DateTime instead of a string.
    # Normalise both sides before comparing or every cycle duplicates the full log.
    $priorCreationUtc = ''
    if ($null -ne $state) {
        try { $priorCreationUtc = ([DateTime]$state.creation_utc).ToUniversalTime().ToString('o') } catch { }
    }
    if ($null -ne $state -and [string]$state.source -eq $active -and $priorCreationUtc -eq $creationUtc) {
        $offset = [long]$state.offset_bytes
    } else { $rollover = ($null -ne $state) }
    $reader = [IO.FileStream]::new($active, [IO.FileMode]::Open, [IO.FileAccess]::Read, [IO.FileShare]::ReadWrite)
    try {
        if ($reader.Length -lt $offset) { $offset = 0; $rollover = $true }
        [void]$reader.Seek($offset, [IO.SeekOrigin]::Begin)
        if ($reader.Length -gt $offset) {
            $segmentPath = Join-Path $segments ('args-' + $now.ToString('yyyyMMddTHHmmssfffffffZ') + '-' + [guid]::NewGuid().ToString('N').Substring(0, 8) + '.log')
            $writer = [IO.FileStream]::new($segmentPath, [IO.FileMode]::CreateNew, [IO.FileAccess]::Write, [IO.FileShare]::None)
            try {
                $reader.CopyTo($writer)
                $writer.Flush($true)
                $deltaBytes = $writer.Length
            } finally { $writer.Dispose() }
        }
        $newOffset = $reader.Position
    } finally { $reader.Dispose() }
    # State advances only after all appended bytes reached their local segment.
    Write-Json $statePath ([ordered]@{ source = $active; creation_utc = $creationUtc; offset_bytes = $newOffset; recorded_utc = $now.ToString('o') })
}
$receipt = [ordered]@{
    schema = 'desktop-commander-argument-log-receipt-v1'
    captured_utc = $now.ToString('o')
    source_dir = $srcDir
    active_file_exists = (Test-Path -LiteralPath $active -PathType Leaf)
    active_segment = $segmentPath
    new_active_bytes = $deltaBytes
    newly_copied_rotated_files = $rotatedNew
    newly_copied_rotated_bytes = $rotatedBytes
    active_rollover_observed = $rollover
    confidentiality = 'RAW_ARGUMENTS_LOCAL_ONLY_NOT_REDACTED'
    completeness = 'Cannot guarantee bytes lost before initial capture or between failed capture/rotation cycles'
}
$receiptPath = Join-Path $receiptDir ('arguments-' + $now.ToString('yyyyMMddTHHmmssfffffffZ') + '.json')
Write-Json $receiptPath $receipt
Write-Output ('ARGUMENT_CAPTURE_RECEIPT=' + $receiptPath)
Write-Output ('ACTIVE_DELTA_BYTES=' + $deltaBytes + ' ROTATED_NEW=' + $rotatedNew)
if ($rollover) { Write-Warning 'Active argument-log rollover detected: inspect rotated inventory for completeness.' }
