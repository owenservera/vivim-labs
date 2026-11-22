<#
.SYNOPSIS
Read-only health acceptance for VIVIM's private Desktop Commander observatory.
.DESCRIPTION
Reports state and receipt freshness; never reads raw argument text, commands, tokens
or model sessions. PowerShell 7+; inspect while the Windows user is signed in.
#>
[CmdletBinding()]
param(
  [Parameter(Mandatory=$true)][string]$ArchiveRoot,
  [ValidateRange(1,120)][int]$CaptureStaleMinutes = 7,
  [ValidateRange(1,168)][int]$AnalysisStaleHours = 30
)
Set-StrictMode -Version Latest
$ErrorActionPreference='Stop'
$root=[IO.Path]::GetFullPath($ArchiveRoot)
if ($root -notmatch '(?i)[\\/]\.private[\\/]') { throw 'PRIVATE_ARCHIVE_ROOT_REQUIRED' }
$fail = [Collections.Generic.List[string]]::new()
$warn = [Collections.Generic.List[string]]::new()
$now=[DateTimeOffset]::UtcNow
$statePath=Join-Path $root 'state/cycle-latest.json'
$latestPath=Join-Path $root 'reports/learning-latest.json'
$hashesPath=Join-Path $root 'index/seen.sha256'
function Add-Fail([string]$message) { $script:fail.Add($message) }
function Add-Warn([string]$message) { $script:warn.Add($message) }
Write-Output ('HEALTH_CHECK_UTC='+$now.ToString('o'))
foreach($spec in @(
  [pscustomobject]@{Name='VIVIM-DesktopCommander-Archive';Alias='CAPTURE'},
  [pscustomobject]@{Name='VIVIM-DesktopCommander-DailyLearning';Alias='LEARNING'}
)) {
  $t=Get-ScheduledTask -TaskName $spec.Name -ErrorAction SilentlyContinue
  if ($null -eq $t) { Add-Fail ('SCHEDULE_MISSING_'+$spec.Alias);continue }
  $s=Get-ScheduledTaskInfo -TaskName $spec.Name
  Write-Output ($spec.Alias+'_TASK_STATE='+$t.State)
  Write-Output ($spec.Alias+'_LAST_RESULT_HEX=0x'+$s.LastTaskResult.ToString('X8'))
  Write-Output ($spec.Alias+'_LAST_RUN_UTC='+$s.LastRunTime.ToUniversalTime().ToString('o'))
  if ($s.LastTaskResult -ne 0 -and $t.State -ne 'Running') {
    Add-Warn ($spec.Alias+'_LAST_TASK_RESULT_NOT_ZERO')
  }
}
if (-not (Test-Path -LiteralPath $statePath -PathType Leaf)) { Add-Fail 'CAPTURE_RECEIPT_MISSING' }
else {
  try {
    $r=Get-Content -LiteralPath $statePath -Raw | ConvertFrom-Json
    $finished=[DateTimeOffset]::Parse([string]$r.finished_utc).ToUniversalTime()
    $lag=($now-$finished).TotalMinutes
    Write-Output ('CAPTURE_RECEIPT_STATUS='+$r.status)
    Write-Output ('CAPTURE_RECEIPT_UTC='+$finished.ToString('o'))
    Write-Output ('CAPTURE_AGE_MINUTES='+[math]::Round($lag,2))
    if ($r.status -ne 'SUCCESS') { Add-Fail 'CAPTURE_LAST_STATUS_NOT_SUCCESS' }
    if ($lag -gt $CaptureStaleMinutes -or $lag -lt -1) { Add-Fail 'CAPTURE_RECEIPT_STALE_OR_FUTURE' }
  } catch { Add-Fail 'CAPTURE_RECEIPT_UNREADABLE' }
}
if (-not (Test-Path -LiteralPath $latestPath -PathType Leaf)) { Add-Fail 'LEARNING_REPORT_MISSING' }
else {
  try {
    $l=Get-Content -LiteralPath $latestPath -Raw | ConvertFrom-Json
    $generated=[DateTimeOffset]::Parse([string]$l.generated_utc).ToUniversalTime()
    $lag=($now-$generated).TotalHours
    Write-Output ('LEARNING_REPORT_UTC='+$generated.ToString('o'))
    Write-Output ('LEARNING_REPORT_AGE_HOURS='+[math]::Round($lag,2))
    Write-Output ('LEARNING_INDEXED_ROWS='+$l.observed_index_rows)
    Write-Output ('LEARNING_UPSTREAM_OMITTED_ROWS='+$l.upstream_large_output_omissions)
    if ($lag -gt $AnalysisStaleHours -or $lag -lt -1) { Add-Warn 'LEARNING_REPORT_STALE_OR_FUTURE' }
  } catch { Add-Fail 'LEARNING_REPORT_UNREADABLE' }
}
if (-not (Test-Path -LiteralPath $hashesPath -PathType Leaf)) { Add-Fail 'UNIQUE_HASH_INDEX_MISSING' }
else {
  $indexCount=0
  foreach($line in [IO.File]::ReadLines($hashesPath)) { if($line.Length -eq 64){$indexCount++} }
  Write-Output ('INDEX_UNIQUE_HASHES='+$indexCount)
}
$disk=[IO.DriveInfo]::new([IO.Path]::GetPathRoot($root))
Write-Output ('ARCHIVE_DISK_FREE_GB='+[math]::Round($disk.AvailableFreeSpace/1GB,2))
if($disk.AvailableFreeSpace -lt 536870912) { Add-Fail 'DISK_FREE_UNDER_512MB' }
elseif($disk.AvailableFreeSpace -lt 2147483648) { Add-Warn 'DISK_FREE_UNDER_2GB' }
Write-Output ('HEALTH_FAILURES='+$fail.Count+' WARNINGS='+$warn.Count)
foreach($item in $fail){Write-Output ('HEALTH_FAIL='+$item)}
foreach($item in $warn){Write-Output ('HEALTH_WARN='+$item)}
if($fail.Count -gt 0){Write-Output 'HEALTH_RESULT=FAIL';exit 1}
if($warn.Count -gt 0){Write-Output 'HEALTH_RESULT=WARN';exit 0}
Write-Output 'HEALTH_RESULT=PASS'
