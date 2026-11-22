<#
.SYNOPSIS
Run one explicitly selected local validation command and preserve complete stdout/stderr.
.DESCRIPTION
No log replay, no model call, no network/upload, no automatic invocation. The caller
must supply executable and arguments. Receipts are metadata only; raw output stays
in a private, Git-ignored folder on the workstation. A successful exit is NOT a
product acceptance claim. Supported PowerShell 7+ and .NET's byte stream APIs.
.EXAMPLE
pwsh -NoProfile -File .\capture-validation-output.ps1 -ArchiveRoot 'C:\path\.private\desktop-commander-stream' -EvidenceId 'ZCODE-READONLY-SITREP' -Surface 'local-ops' -SourceSha 'UNPINNED-OPS' -Executable 'C:\Program Files\PowerShell\7\pwsh.exe' -CommandArguments @('-NoProfile','-File','C:\path\scripts\sitrep.ps1')
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory=$true)][string]$ArchiveRoot,
    [Parameter(Mandatory=$true)][string]$EvidenceId,
    [Parameter(Mandatory=$true)][ValidateSet('local-ops','gpui','pwa','host','rust-core','android','unknown')][string]$Surface,
    [Parameter(Mandatory=$true)][string]$SourceSha,
    [Parameter(Mandatory=$true)][string]$Executable,
    [string[]]$CommandArguments = @(),
    [ValidateRange(1,1800)][int]$TimeoutSeconds = 180
)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
if ($EvidenceId -notmatch '^[A-Za-z0-9][A-Za-z0-9_.-]{2,79}$') { throw 'INVALID_EVIDENCE_ID' }
if ($SourceSha -notmatch '^(?i:[a-f0-9]{40})$' -and $SourceSha -ne 'UNPINNED-OPS') { throw 'SHA_REQUIRED_OR_EXPLICIT_UNPINNED_OPS' }
if ($SourceSha -eq 'UNPINNED-OPS' -and $Surface -ne 'local-ops') { throw 'PRODUCT_TEST_REQUIRES_EXACT_SOURCE_SHA' }
$root = [IO.Path]::GetFullPath($ArchiveRoot)
if ($root -notmatch '(?i)[\\/]\.private[\\/]') { throw 'PRIVATE_ARCHIVE_ROOT_REQUIRED' }
if (-not [IO.Directory]::Exists($root)) { throw 'ARCHIVE_ROOT_NOT_FOUND' }
$exe = (Resolve-Path -LiteralPath $Executable -ErrorAction Stop).ProviderPath
# Explicit test runners, never generic cmd.exe/powershell.exe shell interpretation.
$approvedExeNames = @('pwsh.exe','node.exe','bun.exe','cargo.exe','rustc.exe','git.exe')
if ([IO.Path]::GetFileName($exe).ToLowerInvariant() -notin $approvedExeNames) {
    throw 'EXECUTABLE_NOT_ALLOWLISTED'
}
$drive = [IO.DriveInfo]::new([IO.Path]::GetPathRoot($root))
if ($drive.AvailableFreeSpace -lt 536870912) { throw 'CAPTURE_REFUSED_UNDER_512MB_FREE' }
$start = [DateTimeOffset]::UtcNow
$stamp = $start.ToString('yyyyMMddTHHmmssfffffffZ')
$runId = $stamp + '-' + $EvidenceId + '-' + [guid]::NewGuid().ToString('N').Substring(0,8)
$runDir = Join-Path (Join-Path $root 'validation-runs') $runId
[void][IO.Directory]::CreateDirectory($runDir)
$stdoutPath = Join-Path $runDir 'stdout.bin'
$stderrPath = Join-Path $runDir 'stderr.bin'
$receiptPath = Join-Path $runDir 'receipt.json'
$psi = [Diagnostics.ProcessStartInfo]::new()
$psi.FileName = $exe
$psi.UseShellExecute = $false
$psi.CreateNoWindow = $true
$psi.RedirectStandardOutput = $true
$psi.RedirectStandardError = $true
foreach ($arg in $CommandArguments) { [void]$psi.ArgumentList.Add($arg) }
$shaInput = [Text.Encoding]::UTF8.GetBytes(($exe + [char]0 + ($CommandArguments -join [char]0)))
$specHash = [Convert]::ToHexString([Security.Cryptography.SHA256]::HashData($shaInput)).ToLowerInvariant()
$process = [Diagnostics.Process]::new()
$process.StartInfo = $psi
$exitCode = $null
$timedOut = $false
$startError = $null
$outFile = $null
$errFile = $null
try {
    $outFile = [IO.FileStream]::new($stdoutPath,[IO.FileMode]::CreateNew,[IO.FileAccess]::Write,[IO.FileShare]::Read)
    $errFile = [IO.FileStream]::new($stderrPath,[IO.FileMode]::CreateNew,[IO.FileAccess]::Write,[IO.FileShare]::Read)
    try {
        if (-not $process.Start()) { throw 'PROCESS_START_RETURNED_FALSE' }
        # Start both byte copies immediately; do not decode, normalize newlines or
        # truncate output above Desktop Commander's 4-KiB history cutoff.
        $stdoutCopy = $process.StandardOutput.BaseStream.CopyToAsync($outFile)
        $stderrCopy = $process.StandardError.BaseStream.CopyToAsync($errFile)
        if (-not $process.WaitForExit($TimeoutSeconds * 1000)) {
            $timedOut = $true
            try { $process.Kill($true) } catch { }
        }
        $process.WaitForExit()
        [Threading.Tasks.Task]::WaitAll(@($stdoutCopy,$stderrCopy))
        $exitCode = $process.ExitCode
    } finally {
        if ($null -ne $outFile) { $outFile.Flush($true); $outFile.Dispose() }
        if ($null -ne $errFile) { $errFile.Flush($true); $errFile.Dispose() }
    }
} catch {
    $startError = $_.Exception.GetType().Name
} finally {
    $process.Dispose()
}
$finish = [DateTimeOffset]::UtcNow
function Get-RecordedFile([string]$p) {
    if (-not (Test-Path -LiteralPath $p -PathType Leaf)) { return $null }
    $item = Get-Item -LiteralPath $p
    return [ordered]@{
        private_path = $p
        bytes = $item.Length
        sha256 = (Get-FileHash -LiteralPath $p -Algorithm SHA256).Hash.ToLowerInvariant()
    }
}
$status = if ($null -ne $startError) {'ERROR'} elseif ($timedOut) {'TIMEOUT'} elseif ($exitCode -eq 0) {'PROCESS_EXIT_ZERO'} else {'PROCESS_EXIT_NONZERO'}
$receipt = [ordered]@{
    schema = 'vivim-private-validation-output-v1'
    evidence_id = $EvidenceId
    run_id = $runId
    started_utc = $start.ToString('o')
    finished_utc = $finish.ToString('o')
    duration_ms = [math]::Round(($finish-$start).TotalMilliseconds)
    surface = $Surface
    source_sha = $SourceSha
    executable_basename = [IO.Path]::GetFileName($exe)
    command_spec_sha256 = $specHash
    args_stored = $false
    status = $status
    process_exit_code = $exitCode
    timeout_seconds = $TimeoutSeconds
    startup_error_class_if_any = $startError
    stdout = (Get-RecordedFile $stdoutPath)
    stderr = (Get-RecordedFile $stderrPath)
    product_acceptance = 'NOT_DETERMINED_BY_PROCESS_EXIT'
    privacy = 'RAW_OUTPUT_PRIVATE_LOCAL_ONLY'
}
[IO.File]::WriteAllText($receiptPath,($receipt | ConvertTo-Json -Depth 6),[Text.UTF8Encoding]::new($false))
Write-Output ('PRIVATE_RECEIPT=' + $receiptPath)
Write-Output ('CAPTURE_STATUS=' + $status)
Write-Output ('PROCESS_EXIT_CODE=' + $exitCode)
Write-Output ('STDOUT_BYTES=' + $receipt.stdout.bytes + ' STDERR_BYTES=' + $receipt.stderr.bytes)
if ($status -ne 'PROCESS_EXIT_ZERO') { exit 1 }
