<#
.SYNOPSIS
Derive a minimal private PWA acceptance receipt from ARBOR's existing canonical runner.
.DESCRIPTION
Reads a private capture-validation-output.ps1 run folder. Never prints or sends
raw stdout/stderr, command arguments or environment data. No model or network call.
Requires the real official PWA runner's final structured JSON result and TAP suites.
#>
[CmdletBinding()]
param(
  [Parameter(Mandatory=$true)][string]$ValidationRunDirectory
)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$runDir = [IO.Path]::GetFullPath($ValidationRunDirectory)
if ($runDir -notmatch '(?i)[\\/]\.private[\\/]') { throw 'PRIVATE_VALIDATION_RUN_REQUIRED' }
$receiptPath = Join-Path $runDir 'receipt.json'
$stdoutPath = Join-Path $runDir 'stdout.bin'
if (-not (Test-Path -LiteralPath $receiptPath -PathType Leaf) -or -not (Test-Path -LiteralPath $stdoutPath -PathType Leaf)) { throw 'MISSING_CAPTURE_EVIDENCE' }
$r = Get-Content -LiteralPath $receiptPath -Raw | ConvertFrom-Json
if ($r.schema -ne 'vivim-private-validation-output-v1' -or $r.surface -ne 'pwa') { throw 'UNSUPPORTED_RECEIPT_SCOPE' }
if ([string]$r.source_sha -notmatch '^[a-fA-F0-9]{40}$') { throw 'EXACT_SHA_REQUIRED' }
$hash = (Get-FileHash -LiteralPath $stdoutPath -Algorithm SHA256).Hash.ToLowerInvariant()
if ($hash -ne [string]$r.stdout.sha256) { throw 'STDOUT_HASH_NOT_MATCHING_RECEIPT' }
$txt = [IO.File]::ReadAllText($stdoutPath, [Text.Encoding]::UTF8)
$marker = 'PWA actual Node results'
$at = $txt.LastIndexOf($marker, [StringComparison]::Ordinal)
if ($at -lt 0) { throw 'CANONICAL_PWA_SUMMARY_NOT_FOUND' }
try {
  $j = ConvertFrom-Json -InputObject $txt.Substring($at+$marker.Length).Trim() -ErrorAction Stop
} catch { throw 'CANONICAL_PWA_SUMMARY_INVALID_JSON' }
if ($j.schema -ne 'arbor.pwa-node-test-result/1') { throw 'UNKNOWN_PWA_SUMMARY_SCHEMA' }
$actual = @($j.results)
$failCodes = @($actual | Where-Object { $null -eq $_.exitCode -or [int]$_.exitCode -ne 0 }).Count
$syntaxResults = @($actual | Where-Object { [string]$_.name -like 'Syntax *' })
$suiteResults = @($actual | Where-Object { [string]$_.name -notlike 'Syntax *' })
if ($suiteResults.Count -lt 1) { throw 'NO_TEST_SUITES_EXECUTED' }
$summary = [ordered]@{}
foreach ($key in @('tests','pass','fail','cancelled','skipped')) {
  $hits = [regex]::Matches($txt, '(?m)^[^\r\n]*\b' + $key + '\s+(\d+)\s*$')
  $values = @($hits | ForEach-Object { [int]$_.Groups[1].Value })
  $summary[$key] = [ordered]@{total=(($values | Measure-Object -Sum).Sum);per_suite=$values}
}
if ($summary.tests.per_suite.Count -ne $suiteResults.Count) { throw 'TAP_SUITE_SUMMARY_COUNT_MISMATCH' }
$tests = [int]$summary.tests.total; $pass = [int]$summary.pass.total; $fail = [int]$summary.fail.total
$accept = ($null -ne $r.process_exit_code -and $r.process_exit_code -eq 0 -and $j.failed -eq 0 -and $failCodes -eq 0 -and $tests -gt 0 -and $pass -eq $tests -and $fail -eq 0 -and $summary.cancelled.total -eq 0)
$report = [ordered]@{
  schema = 'vivim-pwa-canonical-code-verification-v1'
  analyzed_utc = [DateTimeOffset]::UtcNow.ToString('o')
  evidence_id = $r.evidence_id
  source_sha = $r.source_sha
  command_spec_sha256 = $r.command_spec_sha256
  stdout_sha256 = $hash
  process_exit_code = $r.process_exit_code
  canonical_command_groups = $actual.Count
  canonical_passed_command_groups = [int]$j.passed
  canonical_failed_command_groups = [int]$j.failed
  syntax_checks = $syntaxResults.Count
  named_test_suites = @($suiteResults | ForEach-Object { [ordered]@{name=$_.name;exit_code=$_.exitCode} })
  tap_tests = $tests
  tap_passed = $pass
  tap_failed = $fail
  tap_cancelled = [int]$summary.cancelled.total
  tap_skipped = [int]$summary.skipped.total
  tap_tests_per_suite = @($summary.tests.per_suite)
  verified_code_test_status = $(if($accept){'PASS'}else{'FAIL'})
  device_visual_evidence = 'NOT_CHECKED'
  remote_ci_status = 'NOT_DETERMINED_FROM_THIS_RUN'
  raw_output_disclosed = $false
}
$path = Join-Path $runDir 'code-verification-summary.json'
[IO.File]::WriteAllText($path, ($report | ConvertTo-Json -Depth 7), [Text.UTF8Encoding]::new($false))
Write-Output ('CODE_VERIFICATION_STATUS=' + $report.verified_code_test_status)
Write-Output ('SOURCE_SHA=' + $report.source_sha)
Write-Output ('COMMAND_GROUPS=' + $report.canonical_passed_command_groups + '/' + $report.canonical_command_groups)
Write-Output ('TAP_TESTS_PASS=' + $pass + '/' + $tests + ' FAILED=' + $fail)
Write-Output ('PRIVATE_SUMMARY_PATH=' + $path)
if (-not $accept) { exit 1 }
