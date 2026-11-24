<#
.SYNOPSIS
Archives Desktop Commander tool-history.jsonl as private local snapshots and metadata-only indexes.
.DESCRIPTION
Explicit source path is required; never scans the workstation or sends data to a server.
Designed for PowerShell 7+. Source history may already have been trimmed, and outputs >4 KiB
may be omitted by Desktop Commander. Captures do not reconstruct absent bytes.
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$Source,
    [string]$ArchiveRoot = (Join-Path (Split-Path -Parent $PSScriptRoot) 'local-logs')
)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
if (-not (Test-Path -LiteralPath $Source -PathType Leaf)) { throw "SOURCE_NOT_FOUND: $Source" }
$sourcePath = [IO.Path]::GetFullPath($Source)
$archivePath = [IO.Path]::GetFullPath($ArchiveRoot)
if ([IO.Path]::GetFileName($sourcePath) -ne 'tool-history.jsonl') { throw 'Expected explicit Desktop Commander tool-history.jsonl; refusing unrelated source.' }
if ($archivePath.StartsWith((Split-Path -Parent $sourcePath) + [IO.Path]::DirectorySeparatorChar, [StringComparison]::OrdinalIgnoreCase)) { throw 'ArchiveRoot may not be nested under the history source directory.' }

function Ensure-Dir([string]$path) {
    [void][IO.Directory]::CreateDirectory($path)
}
function Text-Sha256([string]$value) {
    $hasher = [Security.Cryptography.SHA256]::Create()
    try {
        $digest = $hasher.ComputeHash([Text.Encoding]::UTF8.GetBytes($value))
        return ([BitConverter]::ToString($digest) -replace '-', '').ToLowerInvariant()
    } finally { $hasher.Dispose() }
}
function Append-Utf8([string]$path, [string]$line) {
    Ensure-Dir (Split-Path -Parent $path)
    [IO.File]::AppendAllText($path, $line + [Environment]::NewLine, [Text.UTF8Encoding]::new($false))
}

$day = (Get-Date).ToUniversalTime().ToString('yyyy-MM-dd')
$rawDir = Join-Path $archivePath ('raw/' + $day)
$indexDir = Join-Path $archivePath 'index'
$receiptsDir = Join-Path $archivePath 'receipts'
Ensure-Dir $rawDir
Ensure-Dir $indexDir
Ensure-Dir $receiptsDir
$staging = Join-Path $rawDir ('capture-' + [guid]::NewGuid().ToString('N') + '.tmp')

# Copy a point-in-time prefix while the original writer may still have the file open.
$read = [IO.FileStream]::new($sourcePath, [IO.FileMode]::Open, [IO.FileAccess]::Read, [IO.FileShare]::ReadWrite)
try {
    $write = [IO.FileStream]::new($staging, [IO.FileMode]::CreateNew, [IO.FileAccess]::Write, [IO.FileShare]::None)
    try {
        $read.CopyTo($write)
        $write.Flush($true)
    } finally { $write.Dispose() }
} finally { $read.Dispose() }

$snapshotSha = (Get-FileHash -LiteralPath $staging -Algorithm SHA256).Hash.ToLowerInvariant()
# Keep the full source copy only while indexing; durable raw storage is delta-only.
# This avoids duplicating a multi-MB bounded history every minute on a low-disk PC.
$snapshotPath = $staging
$snapshotBytes = (Get-Item -LiteralPath $staging).Length
$segmentPath = Join-Path $rawDir ('segment-' + (Get-Date).ToUniversalTime().ToString('yyyyMMddTHHmmssfffffffZ') + '-' + $snapshotSha.Substring(0, 12) + '.jsonl')

$seenPath = Join-Path $indexDir 'seen.sha256'
$seen = [Collections.Generic.HashSet[string]]::new([StringComparer]::OrdinalIgnoreCase)
if (Test-Path -LiteralPath $seenPath) {
    foreach ($id in [IO.File]::ReadLines($seenPath)) { if ($id.Length -eq 64) { [void]$seen.Add($id) } }
}
$total = 0; $new = 0; $duplicates = 0; $malformed = 0; $omitted = 0
$firstTimestamp = $null; $lastTimestamp = $null
foreach ($line in [IO.File]::ReadLines($snapshotPath)) {
    if ([string]::IsNullOrWhiteSpace($line)) { continue }
    $total++
    $id = Text-Sha256 $line
    if ($seen.Contains($id)) { $duplicates++; continue }
    try { $rec = ConvertFrom-Json -InputObject $line -ErrorAction Stop } catch { $malformed++; continue }
    $name = [string]$rec.toolName
    if ([string]::IsNullOrWhiteSpace($name)) { $name = 'unknown' }
    $safeName = ($name -replace '[^a-zA-Z0-9_-]', '_')
    $timestampRaw = [string]$rec.timestamp
    $stampUtc = $null
    if ($timestampRaw) {
        try {
            $stampUtc = ([DateTimeOffset]::Parse($timestampRaw, [Globalization.CultureInfo]::GetCultureInfo('en-US'))).ToUniversalTime().ToString('o')
        } catch { }
    }
    if ($null -eq $firstTimestamp) { $firstTimestamp = $stampUtc }
    $lastTimestamp = $stampUtc
    $eventDay = if ($stampUtc) { $stampUtc.Substring(0, 10) } else { $day }
    $text = ''
    if ($null -ne $rec.output) {
        # In-memory inspection only. Raw arguments and raw output are never placed in indexes.
        $text = ($rec.output | ConvertTo-Json -Depth 20 -Compress)
    }
    $isOmitted = $text.Contains('[output omitted from history:')
    if ($isOmitted) { $omitted++ }
    $exit = $null
    if ($text -match 'completed with exit code\s+(-?[0-9]+)') {
        $parsedExit = [long]0
        if ([long]::TryParse($Matches[1], [ref]$parsedExit)) { $exit = $parsedExit }
    }
    $row = [ordered]@{
        schema = 'desktop-commander-index-v1'
        timestamp_utc = $stampUtc
        timestamp_original = $timestampRaw
        tool = $name
        observed_source_line_sha256 = $id
        source_snapshot_sha256 = $snapshotSha
        process_exit_code_if_present = $exit
        process_output_omitted = $isOmitted
        tool_duration_ms_if_present = $rec.duration
        evidence_scope = 'metadata_only_no_raw_args_or_outputs'
    }
    $json = $row | ConvertTo-Json -Compress -Depth 5
    # Store raw source exactly once, privately, without duplicating the rolling file.
    Append-Utf8 $segmentPath $line
    Append-Utf8 (Join-Path $indexDir ('days/' + $eventDay + '.jsonl')) $json
    Append-Utf8 (Join-Path $indexDir ('tools/' + $safeName + '/' + $eventDay + '.jsonl')) $json
    Append-Utf8 $seenPath $id
    [void]$seen.Add($id)
    $new++
}
Remove-Item -LiteralPath $staging -Force
$segmentBytes = if (Test-Path -LiteralPath $segmentPath) { (Get-Item -LiteralPath $segmentPath).Length } else { 0 }
$receipt = [ordered]@{
    schema = 'desktop-commander-capture-receipt-v1'
    captured_utc = (Get-Date).ToUniversalTime().ToString('o')
    source = $sourcePath
    observed_source_sha256 = $snapshotSha
    observed_source_bytes = $snapshotBytes
    private_raw_delta_segment = if ($segmentBytes -gt 0) { $segmentPath } else { $null }
    private_raw_delta_bytes = $segmentBytes
    lines_observed = $total
    records_added_to_index = $new
    duplicates_skipped = $duplicates
    malformed_lines_not_indexed = $malformed
    omitted_outputs_in_new_records = $omitted
    first_new_timestamp_utc = $firstTimestamp
    last_new_timestamp_utc = $lastTimestamp
    historic_completeness = 'UNPROVEN: Desktop Commander trims history and omits large outputs'
}
$receiptPath = Join-Path $receiptsDir ('capture-' + (Get-Date).ToUniversalTime().ToString('yyyyMMddTHHmmssfffffffZ') + '-' + $snapshotSha.Substring(0,12) + '.json')
[IO.File]::WriteAllText($receiptPath, ($receipt | ConvertTo-Json -Depth 4), [Text.UTF8Encoding]::new($false))
Write-Output ('CAPTURE_RECEIPT=' + $receiptPath)
Write-Output ('OBSERVED_SOURCE_SHA256=' + $snapshotSha)
Write-Output ('RAW_DELTA_BYTES=' + $segmentBytes)
Write-Output ('OBSERVED=' + $total + ' NEW=' + $new + ' DUPLICATE=' + $duplicates + ' MALFORMED=' + $malformed + ' LARGE_OUTPUT_OMITTED=' + $omitted)
if ($malformed -gt 0) { Write-Warning 'Malformed/incomplete source lines detected; source may have been mid-write. Retry capture later.' }
