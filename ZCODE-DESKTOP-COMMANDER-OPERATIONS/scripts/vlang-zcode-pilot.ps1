# A bounded one-per-SHA ZCode dynamic-workflow pilot. Never scheduled automatically.
# Child is an exact ZCode CLI invocation; parent monitors 429/timeout and stops
# only its own process tree. Never prints private model/provider credentials.
[CmdletBinding()]
param(
  [ValidateSet('status','launch','child')][string]$Action='status',
  [string]$Sha='',
  [string]$AdmissionReceipt=''
)
$ErrorActionPreference='Stop'
$root='C:\0-BlackBoxProject-0\ARBOR'
$logRoot=Join-Path $PSScriptRoot '..\local-logs\vlang-model'
New-Item -ItemType Directory -Path $logRoot -Force | Out-Null
# Retains existing ZCode provider registry; only validates independently
# supplied exact-source health, quota and owner-approved billing metadata.
function Assert-ModelAdmission {
  param([string]$SelectedSha,[string]$ReceiptPath)
  if([string]::IsNullOrWhiteSpace($ReceiptPath) -or
     -not [IO.Path]::IsPathRooted($ReceiptPath) -or
     -not (Test-Path -LiteralPath $ReceiptPath -PathType Leaf)){
    throw 'MODEL_ADMISSION_RECEIPT_REQUIRED'
  }
  $gate=Join-Path $root 'scripts\zcode-model-session-admission.mjs'
  if(-not (Test-Path -LiteralPath $gate -PathType Leaf)){
    throw 'MODEL_ADMISSION_GATE_UNAVAILABLE'
  }
  & node $gate --verify $ReceiptPath $SelectedSha
  if($LASTEXITCODE -ne 0){throw 'MODEL_ADMISSION_UNVERIFIED'}
}
if($Action -eq 'child'){
  if($Sha -notmatch '^[a-f0-9]{40}$'){throw 'MISSING_EXACT_SOURCE_SHA'}
  $childHead=(& git -C $root rev-parse HEAD).Trim()
  $childUp=(& git -C $root rev-parse refs/remotes/origin/main).Trim()
  $childDirty=@(& git -C $root status --porcelain --untracked-files=all)
  if($childHead -ne $Sha -or $childHead -ne $childUp -or $childDirty.Count -gt 0){
    throw 'CHILD_SOURCE_SHA_OR_TREE_MISMATCH'
  }
  Assert-ModelAdmission -SelectedSha $Sha -ReceiptPath $AdmissionReceipt
  Set-Location $root
  $prompt="Load the official dynamic-workflows skill. Invoke CreateWorkflow by EXISTING project path .zcode/workflows/arbor-vlang-iterative-observers.dwf.ts with arguments sha=$Sha. Read-only source and test verification only; no file writes, Host actions, browser or provider sends. Do not retry model 429 or select paid fallback. Report the actual run ID or exact error, not a plan."
  & zcode --mode plan --workflow-mode alwaysOn --agent-always-background on --cwd $root --prompt $prompt
  exit $LASTEXITCODE
}
$latest='C:\0-BlackBoxProject-0\vivim-labs\ZCODE-DESKTOP-COMMANDER-OPERATIONS\local-logs\vlang-iterations\latest.json'
$head=(& git -C $root rev-parse HEAD).Trim()
$up=(& git -C $root rev-parse refs/remotes/origin/main).Trim()
$dirty=@(& git -C $root status --porcelain --untracked-files=all)
if($head -notmatch '^[a-f0-9]{40}$' -or $head -ne $up -or $dirty.Count -gt 0){throw 'BLOCKED_UNPINNED_SOURCE'}
$marker=Join-Path $logRoot ($head+'.json')
if($Action -eq 'status'){
  Write-Output ('SOURCE_SHA='+$head)
  if(Test-Path $marker){$v=Get-Content $marker -Raw|ConvertFrom-Json;Write-Output ('PILOT='+$v.state+' RUN_ID='+$v.runId+' at '+$v.startedAt)}
  else{Write-Output 'PILOT_NOT_ATTEMPTED_FOR_SHA'}
  return
}
if(-not (Test-Path $latest)){throw 'FIRST_RUN_DETERMINISTIC_VERIFIER'}
$verified=Get-Content $latest -Raw|ConvertFrom-Json
if($verified.sha -ne $head -or $verified.state -ne 'PASS'){throw 'DETERMINISTIC_BASELINE_NOT_VERIFIED'}
Assert-ModelAdmission -SelectedSha $head -ReceiptPath $AdmissionReceipt
if(Test-Path $marker){throw 'MODEL_PILOT_ALREADY_ATTEMPTED_FOR_SHA_NO_AUTO_RETRY'}
$state=@{sha=$head;startedAt=(Get-Date).ToUniversalTime().ToString('o');state='ADMITTED';runId=$null}
$state|ConvertTo-Json|Set-Content -LiteralPath $marker -Encoding utf8
$stdout=Join-Path $logRoot ($head+'.stdout.log')
$stderr=Join-Path $logRoot ($head+'.stderr.log')
$argv=@('-NoProfile','-NonInteractive','-ExecutionPolicy','Bypass','-File',('"{0}"' -f $PSCommandPath),'-Action','child','-Sha',$head,'-AdmissionReceipt',('"{0}"' -f $AdmissionReceipt))
$p=Start-Process -FilePath 'powershell.exe' -ArgumentList $argv -PassThru -WindowStyle Hidden -RedirectStandardOutput $stdout -RedirectStandardError $stderr
$deadline=(Get-Date).AddSeconds(100)
$reason=$null
while(-not $p.HasExited){
  Start-Sleep -Seconds 2;$p.Refresh()
  if(Test-Path $stderr){
    $tail=(Get-Content $stderr -Tail 40 -ErrorAction SilentlyContinue) -join ' '
    if($tail -match 'statusCode: 429|rate_limit_error|tpm_rate_limit_exceeded'){
      $reason='BLOCKED_RATE_LIMIT';break
    }
  }
  if((Get-Date) -ge $deadline){$reason='BLOCKED_TIMEOUT';break}
}
if($reason){
  # Taskkill targets ONLY the PowerShell process just spawned and its child tree.
  & taskkill.exe /PID $p.Id /T /F 2>$null | Out-Null
  $state.state=$reason
}else{
  $state.state=if($p.ExitCode -eq 0){'CLI_EXIT_0_UNVERIFIED_WORKFLOW'}else{'CLI_FAILED'}
}
$events=((Get-Content $stderr -Tail 150 -ErrorAction SilentlyContinue) -join ' ')
$match=[regex]::Match($events,'dwfrun-[a-f0-9-]{36}')
if($match.Success){$state.runId=$match.Value}
$state.finishedAt=(Get-Date).ToUniversalTime().ToString('o')
$state|ConvertTo-Json|Set-Content -LiteralPath $marker -Encoding utf8
Write-Output ('MODEL_PILOT='+$state.state+' RUN_ID='+$state.runId+' SHA='+$head)
if($state.state -ne 'CLI_EXIT_0_UNVERIFIED_WORKFLOW'){exit 1}
