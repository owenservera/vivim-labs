<#
.SYNOPSIS
Summarize private Desktop Commander metadata into safe local trend reports.
.DESCRIPTION
No provider, API, network access, or execution of logged commands. Pattern matching
happens on private raw tool-history records in memory, with aggregate counts only.
#>
[CmdletBinding()]
param([Parameter(Mandatory=$true)][string]$ArchiveRoot)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$idxDir = Join-Path $ArchiveRoot 'index/days'
$rawDir = Join-Path $ArchiveRoot 'raw'
$reportDir = Join-Path $ArchiveRoot 'reports'
if (-not (Test-Path -LiteralPath $idxDir -PathType Container)) { throw 'INDEX_NOT_FOUND' }
[void][IO.Directory]::CreateDirectory($reportDir)
$records = [Collections.Generic.List[object]]::new()
foreach ($file in @(Get-ChildItem -LiteralPath $idxDir -File -Filter '*.jsonl' -ErrorAction Stop)) {
  foreach ($line in [IO.File]::ReadLines($file.FullName)) {
    if (-not [string]::IsNullOrWhiteSpace($line)) {
      try { $records.Add((ConvertFrom-Json -InputObject $line -ErrorAction Stop)) } catch { }
    }
  }
}
$groups = @($records | Group-Object -Property tool | Sort-Object -Property Count -Descending)
$toolRows = @($groups | ForEach-Object { [ordered]@{tool=$_.Name;observations=$_.Count} })
$omitted = @($records | Where-Object {$_.process_output_omitted -eq $true}).Count
$badExit = @($records | Where-Object {$null -ne $_.process_exit_code_if_present -and [int]$_.process_exit_code_if_present -ne 0}).Count
$okExit = @($records | Where-Object {$null -ne $_.process_exit_code_if_present -and [int]$_.process_exit_code_if_present -eq 0}).Count
$patterns = [ordered]@{
  powershell_profile_noise = 0
  pwa_offline_asset_assertion = 0
  wiki_typing_assertion = 0
  http_401_403_test_mismatch = 0
  git_dirty_build_guard = 0
  model_rate_limit = 0
  precheck_timeout = 0
}
$scannedRaw = 0
foreach ($file in @(Get-ChildItem -LiteralPath $rawDir -Recurse -File -Filter 'segment-*.jsonl' -ErrorAction Stop)) {
  foreach ($line in [IO.File]::ReadLines($file.FullName)) {
    if ([string]::IsNullOrWhiteSpace($line)) { continue }
    try { $rec=ConvertFrom-Json -InputObject $line -ErrorAction Stop } catch { continue }
    $scannedRaw++
    if ($null -eq $rec.output) { continue }
    $o = $rec.output | ConvertTo-Json -Compress -Depth 20
    if ($o -match 'Set-PSReadLineOption') { $patterns.powershell_profile_noise++ }
    if ($o -match 'all PWA module imports exist|install precaches every shell asset|install resolves shell paths') { $patterns.pwa_offline_asset_assertion++ }
    if ($o -match 'Wiki-first typing automatically updates') { $patterns.wiki_typing_assertion++ }
    if ($o -match 'actual:\s*403' -and $o -match 'expected:\s*401') { $patterns.http_401_403_test_mismatch++ }
    if ($o -match 'Tracked/untracked workspace is not clean') { $patterns.git_dirty_build_guard++ }
    if ($o -match '(?i)(rate.limit|too.many.requests|HTTP 429|upstream 429)') { $patterns.model_rate_limit++ }
    if ($o -match 'PowerShell did not answer in 15 seconds') { $patterns.precheck_timeout++ }
  }
}
$report = [ordered]@{
  schema = 'dc-local-learning-observatory-v1'
  generated_utc = [DateTimeOffset]::UtcNow.ToString('o')
  scope = 'locally preserved unique tool-history records, not all historical calls'
  observed_index_rows = $records.Count
  upstream_large_output_omissions = $omitted
  process_exit_zero_entries = $okExit
  process_exit_nonzero_entries = $badExit
  raw_history_records_scanned = $scannedRaw
  pattern_hit_entries = $patterns
  top_tool_events = @($toolRows | Select-Object -First 12)
  warning = 'Pattern hits and nonzero exit rows are event observations, NOT unique failing jobs. Counts can understate output-truncated records.'
}
$json = $report | ConvertTo-Json -Depth 10
$summary = @(
  '# Desktop Commander — private evidence trends',
  '',
  ('Generated UTC: ' + $report.generated_utc),
  '',
  ('Indexed unique history rows: **' + $records.Count + '**'),
  ('Upstream omitted-output rows: **' + $omitted + '**'),
  ('Entries with process exit 0: **' + $okExit + '**'),
  ('Entries with process exit nonzero: **' + $badExit + '**'),
  '',
  '## Private source pattern counts (observation rows, not unique incidents)'
)
foreach ($kv in $patterns.GetEnumerator()) { $summary += ('- ' + $kv.Key + ': ' + $kv.Value) }
$summary += @('', '## Most-observed tool types')
foreach ($tool in ($toolRows | Select-Object -First 10)) { $summary += ('- ' + $tool.tool + ': ' + $tool.observations) }
$summary += @('', 'These counts are incomplete when upstream omitted tool outputs. The raw logs stay on this PC and are never included in this report.', '', 'Proposed learning path: reproduce an observed pattern at its exact ARBOR SHA; file a bounded claim in existing TASK-CLAIMS / TEST-LEDGER; ChatGPT cloud fixes source and records a new test receipt.')
$stamp = (Get-Date).ToUniversalTime().ToString('yyyyMMddTHHmmssfffffffZ')
$jsonPath = Join-Path $reportDir ('learning-' + $stamp + '.json')
$mdPath = Join-Path $reportDir ('learning-' + $stamp + '.md')
[IO.File]::WriteAllText($jsonPath, $json, [Text.UTF8Encoding]::new($false))
[IO.File]::WriteAllLines($mdPath, $summary, [Text.UTF8Encoding]::new($false))
Write-Output ('METADATA_LEARNING_JSON=' + $jsonPath)
Write-Output ('METADATA_LEARNING_MD=' + $mdPath)
Write-Output ('OBSERVATIONS=' + $records.Count + ' OMITTED=' + $omitted + ' NONZERO_EXIT_ENTRIES=' + $badExit)
