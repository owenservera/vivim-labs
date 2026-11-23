# VIVIM Labs: one bounded source-pinned native + four-lane verification cycle.
# Runs only in the existing Windows observer schedule. Does not touch the live
# Host, GPUI, Chrome profile, provider accounts, personal corpus or ARBOR source.
# This script prepares an ISOLATED reusable Git worktree and uses the incumbent
# vlang-parallel-cycle.ts runner; it is not a new model router or test engine.
[CmdletBinding()]
param()
$ErrorActionPreference = 'Stop'
$root = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..\..\..')).Path
$arbor = Join-Path $root 'ARBOR'
$checkout = Join-Path $root '.arbor-verifier-source'
$target = Join-Path $root '.arbor-verifier-target'
$runner = Join-Path $PSScriptRoot 'vlang-parallel-cycle.ts'
$latest = Join-Path $PSScriptRoot '..\local-logs\vlang-iterations\latest.json'
$journalDir = Join-Path $PSScriptRoot '..\local-logs\native-verifier'
$journal = Join-Path $journalDir 'latest.json'
New-Item -ItemType Directory -Path $journalDir -Force | Out-Null
$mutex = New-Object System.Threading.Mutex($false, 'Local\VIVIMSourcePinnedNativeVerifier')
$acquired = $mutex.WaitOne(0)
if (-not $acquired) {
  $mutex.Dispose()
  Write-Output 'BLOCKED_ALREADY_RUNNING'
  exit 4
}
$receipt = @{
  schema = 'vivim.native-parallel-verification/1'
  startedAt = (Get-Date).ToUniversalTime().ToString('o')
  state = 'BLOCKED'
  sourceSha = $null
  nativeSha256 = $null
  testReceipt = $null
  verificationContract = 'native-sha-bound/1'
  runnerSha256 = $null
  sourceWrites = 0
  hostEffects = 0
  providerSends = 0
  releasePromoted = $false
  note = 'Offline development verification only; Cargo lock resolution is not release reproducibility proof'
}
$exitCode = 1
try {
  if (-not (Test-Path -LiteralPath (Join-Path $arbor '.git')) -or
      -not (Test-Path -LiteralPath $runner -PathType Leaf)) {
    throw 'REQUIRED_EXISTING_WORKSPACE_OR_RUNNER_MISSING'
  }
  # Fetch without switching, pulling, cleaning or resetting the owner's ARBOR checkout.
  & git -C $arbor fetch --quiet origin main
  if ($LASTEXITCODE -ne 0) { throw 'CANONICAL_FETCH_FAILED' }
  $sha = (& git -C $arbor rev-parse refs/remotes/origin/main).Trim()
  if ($LASTEXITCODE -ne 0 -or $sha -notmatch '^[a-f0-9]{40}$') {
    throw 'CANONICAL_SHA_UNAVAILABLE'
  }
  $receipt.sourceSha = $sha
  if (-not (Test-Path -LiteralPath $checkout)) {
    & git -C $arbor worktree add --detach $checkout $sha
    if ($LASTEXITCODE -ne 0) { throw 'ISOLATED_WORKTREE_CREATE_FAILED' }
  } elseif (-not (Test-Path -LiteralPath (Join-Path $checkout '.git') -PathType Leaf)) {
    throw 'ISOLATED_PATH_EXISTS_BUT_IS_NOT_GIT_WORKTREE'
  }
  $top = (& git -C $checkout rev-parse --show-toplevel).Trim()
  if ($LASTEXITCODE -ne 0 -or
      [IO.Path]::GetFullPath($top).TrimEnd('\') -ine
      [IO.Path]::GetFullPath($checkout).TrimEnd('\')) {
    throw 'UNEXPECTED_WORKTREE_IDENTITY'
  }
  $dirty = @(& git -C $checkout status --porcelain --untracked-files=no)
  if ($LASTEXITCODE -ne 0 -or $dirty.Count -gt 0) {
    throw 'ISOLATED_WORKTREE_TRACKED_CHANGES_EXIST'
  }
  & git -C $checkout checkout --detach --quiet $sha
  if ($LASTEXITCODE -ne 0) { throw 'ISOLATED_PIN_FAILED' }
  $head = (& git -C $checkout rev-parse HEAD).Trim()
  $upstream = (& git -C $checkout rev-parse refs/remotes/origin/main).Trim()
  if ($head -ne $sha -or $upstream -ne $sha) { throw 'ISOLATED_SOURCE_MISMATCH' }

  # Build away from the live target/debug executable, which Windows may have locked.
  # Cargo caching makes same-source runs cheap. Never copy into the live ARBOR tree.
  $manifest = Join-Path $checkout 'core\programmability\Cargo.toml'
  & cargo build --manifest-path $manifest --bin vivim-product-pure --offline --target-dir $target
  if ($LASTEXITCODE -ne 0) { throw 'ISOLATED_NATIVE_BUILD_FAILED' }
  $built = Join-Path $target 'debug\vivim-product-pure.exe'
  $staged = Join-Path $checkout 'target\debug\vivim-product-pure.exe'
  if (-not (Test-Path -LiteralPath $built -PathType Leaf)) {
    throw 'NATIVE_BINARY_MISSING'
  }
  New-Item -ItemType Directory -Path (Split-Path -Parent $staged) -Force | Out-Null
  Copy-Item -LiteralPath $built -Destination $staged -Force
  $sourceHash = (Get-FileHash -LiteralPath $built -Algorithm SHA256).Hash
  $stagedHash = (Get-FileHash -LiteralPath $staged -Algorithm SHA256).Hash
  if ($sourceHash -ne $stagedHash) { throw 'STAGED_NATIVE_HASH_MISMATCH' }
  $receipt.nativeSha256 = $stagedHash
  $receipt.runnerSha256 = (Get-FileHash -LiteralPath $runner -Algorithm SHA256).Hash
  # Do not inherit a PASS for the same source if the compiled executable or
  # test runner changed. The pre-contract journal intentionally forces a replay.
  $prior = $null
  if (Test-Path -LiteralPath $journal -PathType Leaf) {
    try { $prior = Get-Content -LiteralPath $journal -Raw | ConvertFrom-Json } catch { }
  }
  $forceTests = ($null -eq $prior -or
    $prior.verificationContract -ne $receipt.verificationContract -or
    $prior.state -ne 'PASS' -or
    $prior.sourceSha -ne $sha -or
    $prior.nativeSha256 -ne $stagedHash -or
    $prior.runnerSha256 -ne $receipt.runnerSha256)

  $oldRoot = [Environment]::GetEnvironmentVariable('VIVIM_ARBOR_ROOT', 'Process')
  try {
    [Environment]::SetEnvironmentVariable('VIVIM_ARBOR_ROOT', $checkout, 'Process')
    $nodeArgs = @('--experimental-strip-types', $runner, '--once')
    if ($forceTests) { $nodeArgs += '--force' }
    & node @nodeArgs
    $testExit = $LASTEXITCODE
  } finally {
    [Environment]::SetEnvironmentVariable('VIVIM_ARBOR_ROOT', $oldRoot, 'Process')
  }
  if ($testExit -ne 0) { throw "PARALLEL_TEST_RUNNER_EXIT_$testExit" }
  if (-not (Test-Path -LiteralPath $latest -PathType Leaf)) {
    throw 'PARALLEL_TEST_RECEIPT_MISSING'
  }
  $result = Get-Content -LiteralPath $latest -Raw | ConvertFrom-Json
  if ($result.sha -ne $sha -or $result.state -ne 'PASS' -or
      $result.lanes.Count -ne 4 -or
      @($result.lanes | Where-Object { $_.status -ne 'PASS' }).Count -ne 0) {
    throw 'PARALLEL_TEST_RECEIPT_INVALID'
  }
  $receipt.testReceipt = $result.runId
  $receipt.state = 'PASS'
  $exitCode = 0
  Write-Output "NATIVE_PARALLEL_VERIFIED sha=$sha native=$stagedHash receipt=$($result.runId)"
} catch {
  $receipt.error = $_.Exception.Message
  Write-Output "NATIVE_PARALLEL_BLOCKED $($receipt.error)"
} finally {
  $receipt.finishedAt = (Get-Date).ToUniversalTime().ToString('o')
  $receipt | ConvertTo-Json -Depth 7 | Set-Content -LiteralPath $journal -Encoding UTF8
  if ($acquired) { $mutex.ReleaseMutex() }
  $mutex.Dispose()
}
exit $exitCode
