<#
.SYNOPSIS
Run a bounded, private Desktop Commander capture cycle using existing Labs archivers.
.DESCRIPTION
No network, external service, UI capture or ARBOR source mutation. Designed for
Windows Task Scheduler running in the logged-in owner's security context.
#>
[CmdletBinding()]
param(
  [string]$SourceDirectory = (Join-Path $env:USERPROFILE '.claude-server-commander'),
  [Parameter(Mandatory=$true)][string]$ArchiveRoot
)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
# Execution beacon distinguishes Scheduler failing to launch from an in-script failure.
$beaconDir = Join-Path $ArchiveRoot 'state'
[void][IO.Directory]::CreateDirectory($beaconDir)
$beacon = [ordered]@{
  schema = 'desktop-commander-cycle-enter-v1'
  entered_utc = [DateTimeOffset]::UtcNow.ToString('o')
  pid = $PID
  user_profile_available = [bool]$env:USERPROFILE
}
[IO.File]::WriteAllText((Join-Path $beaconDir 'cycle-entered.json'), ($beacon | ConvertTo-Json -Depth 3), [Text.UTF8Encoding]::new($false))
$mutex = [System.Threading.Mutex]::new($false, 'Local\VIVIM_DesktopCommander_HistoryArchive')
$acquired = $false
try {
  $acquired = $mutex.WaitOne(0)
  if (-not $acquired) {
    [IO.File]::WriteAllText((Join-Path $beaconDir 'cycle-skipped.txt'), ([DateTimeOffset]::UtcNow.ToString('o') + ' ALREADY_RUNNING'), [Text.UTF8Encoding]::new($false))
    Write-Output 'CAPTURE_SKIP=ALREADY_RUNNING'
    exit 0
  }
  $histFile = Join-Path $SourceDirectory 'tool-history.jsonl'
  if (-not (Test-Path -LiteralPath $histFile -PathType Leaf)) { throw 'DC_HISTORY_NOT_FOUND' }
  $histScript = Join-Path $PSScriptRoot 'archive-desktop-commander-history.ps1'
  $argsScript = Join-Path $PSScriptRoot 'archive-desktop-commander-arguments.ps1'
  if (-not ((Test-Path -LiteralPath $histScript) -and (Test-Path -LiteralPath $argsScript))) { throw 'ARCHIVER_SCRIPTS_NOT_FOUND' }
  $driveRoot = [IO.Path]::GetPathRoot([IO.Path]::GetFullPath($ArchiveRoot))
  $disk = [IO.DriveInfo]::new($driveRoot)
  if ($disk.AvailableFreeSpace -lt 268435456) { throw 'ARCHIVE_SKIPPED_LOW_DISK_BELOW_256MB' }
  $started = [DateTimeOffset]::UtcNow.ToString('o')
  & $histScript -Source $histFile -ArchiveRoot $ArchiveRoot
  if (-not $?) { throw 'HISTORY_ARCHIVER_ERROR' }
  & $argsScript -SourceDirectory $SourceDirectory -ArchiveRoot $ArchiveRoot
  if (-not $?) { throw 'ARGUMENT_ARCHIVER_ERROR' }
  $statusDir = Join-Path $ArchiveRoot 'state'
  [void][IO.Directory]::CreateDirectory($statusDir)
  $latest = Join-Path $statusDir 'cycle-latest.json'
  $temp = $latest + '.' + [guid]::NewGuid().ToString('N') + '.tmp'
  $status = [ordered]@{
    schema = 'desktop-commander-cycle-v1'
    started_utc = $started
    finished_utc = [DateTimeOffset]::UtcNow.ToString('o')
    status = 'SUCCESS'
    source_history_bytes = (Get-Item -LiteralPath $histFile).Length
    source_directory = $SourceDirectory
    archive_directory = $ArchiveRoot
    disk_free_bytes = $disk.AvailableFreeSpace
    quality = 'History bounded; large outputs may be omitted by upstream'
  }
  [IO.File]::WriteAllText($temp, ($status | ConvertTo-Json -Depth 5), [Text.UTF8Encoding]::new($false))
  Move-Item -LiteralPath $temp -Destination $latest -Force
  Write-Output 'CAPTURE_CYCLE=SUCCESS'
} catch {
  $errorInfo = [ordered]@{
    schema = 'desktop-commander-cycle-error-v1'
    observed_utc = [DateTimeOffset]::UtcNow.ToString('o')
    error = ($_.Exception.Message -replace '(?i)(token|key|secret)=[^\\s]+', '$1=[redacted]')
  }
  [IO.File]::WriteAllText((Join-Path $beaconDir 'cycle-last-error.json'), ($errorInfo | ConvertTo-Json -Depth 3), [Text.UTF8Encoding]::new($false))
  Write-Error 'CAPTURE_CYCLE_FAILED: see private state/cycle-last-error.json' -ErrorAction Continue
  exit 1
} finally {
  if ($acquired) { $mutex.ReleaseMutex() }
  $mutex.Dispose()
}
