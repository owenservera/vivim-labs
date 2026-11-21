param([Parameter(Mandatory=$true)][string]$Sha)
$ErrorActionPreference = 'Stop'
# One-UI owner cutover. This script modifies no application source or dirty checkout.
# Explicitly pins a previously built immutable GitHub main archive and preserves rollback.
if($Sha -notmatch '^[0-9a-f]{40}$'){ throw 'INVALID_COMMIT_SHA' }
$base='C:\0-BlackBoxProject-0\.arbor-pwa-releases'
$release=Join-Path $base $Sha.Substring(0,12)
$receipt=Join-Path $base ($Sha.Substring(0,12)+'.build-ok.txt')
$newScript=Join-Path $release 'apps\vivim-host\server.mjs'
$appJs=Join-Path $release 'apps\mobile-relay\app.js'
if(!(Test-Path $receipt) -or !(Test-Path $newScript) -or !(Test-Path $appJs)){
  throw 'PINNED_RELEASE_NOT_BUILT'
}
if((Get-Content $receipt -Raw) -notmatch [regex]::Escape('sha='+$Sha)){
  throw 'PINNED_BUILD_RECEIPT_MISMATCH'
}
if((Get-Content $appJs -Raw) -notmatch [regex]::Escape('const FIRST_CONTACT = transport.direct && !HOST_PREVIEW;')){
  throw 'SINGLE_UI_DEFAULT_MISSING'
}
Push-Location $release
try{
  & node --test 'apps/mobile-relay/test/first-customer.test.mjs'
  if($LASTEXITCODE -ne 0){ throw 'FIRST_CONTACT_REGRESSION_FAILED' }
}finally{ Pop-Location }
if([string]::IsNullOrWhiteSpace($env:VIVIM_DIRECT_ORIGIN) -or
   [string]::IsNullOrWhiteSpace($env:VIVIM_DIRECT_USER)){
  throw 'HOST_IDENTITY_NOT_CONFIGURED'
}
$listener=Get-NetTCPConnection -State Listen -LocalPort 43187 -ErrorAction Stop |
  Where-Object LocalAddress -eq '127.0.0.1' | Select-Object -First 1
if(!$listener){throw 'CURRENT_HOST_NOT_RUNNING'}
$oldPid=[int]$listener.OwningProcess
$oldProc=Get-CimInstance Win32_Process -Filter ('ProcessId='+$oldPid)
if(!$oldProc -or $oldProc.Name -ne 'node.exe'){throw 'UNEXPECTED_HOST_PROCESS'}
$match=[regex]::Match($oldProc.CommandLine,
  '(?i)([A-Z]:\\0-BlackBoxProject-0\\[^"]*?\\apps[\\/]+vivim-host[\\/]+server\.mjs)')
if(!$match.Success){throw 'UNRECOGNIZED_HOST_SCRIPT'}
$oldScript=$match.Groups[1].Value
$allowed = @(
  'C:\0-BlackBoxProject-0\.arbor-gpui-releases\',
  'C:\0-BlackBoxProject-0\.arbor-pwa-releases\',
  'C:\0-BlackBoxProject-0\ARBOR-first-contact\'
)
if(!($allowed | Where-Object { $oldScript.StartsWith($_,[StringComparison]::OrdinalIgnoreCase) })){
  throw 'UNEXPECTED_HOST_LOCATION'
}
if(!(Test-Path $oldScript)){throw 'OLD_HOST_ROLLBACK_SCRIPT_MISSING'}
$oldRoot=Split-Path (Split-Path (Split-Path $oldScript -Parent) -Parent) -Parent
$pre=Invoke-WebRequest -Uri 'http://127.0.0.1:43187/mobile/index.html?first=1' -UseBasicParsing -TimeoutSec 6
if($pre.StatusCode -ne 200){throw 'OLD_HOST_UNHEALTHY_NO_SWITCH'}
$node=(Get-Command node -ErrorAction Stop).Source
$stdout=Join-Path $base ($Sha.Substring(0,12)+'.one-ui.stdout.log')
$stderr=Join-Path $base ($Sha.Substring(0,12)+'.one-ui.stderr.log')
Write-Output ('CUTOVER_BEGIN sha='+$Sha+' oldHostPid='+$oldPid)
$newProc=$null
try {
  Stop-Process -Id $oldPid -Force -ErrorAction Stop
  Start-Sleep -Milliseconds 750
  $newProc=Start-Process -FilePath $node -ArgumentList ('"'+$newScript+'"') -WorkingDirectory $release -PassThru -RedirectStandardOutput $stdout -RedirectStandardError $stderr
  $healthy=$false
  for($i=0;$i -lt 25;$i++){
    Start-Sleep -Seconds 1
    $newProc.Refresh()
    if($newProc.HasExited){break}
    try {
      $page=Invoke-WebRequest -Uri 'http://127.0.0.1:43187/mobile/index.html' -UseBasicParsing -TimeoutSec 3
      $source=Invoke-WebRequest -Uri 'http://127.0.0.1:43187/mobile/app.js' -UseBasicParsing -TimeoutSec 3
      if($page.StatusCode -ne 200 -or $source.StatusCode -ne 200 -or
        $source.Content -notmatch [regex]::Escape('const FIRST_CONTACT = transport.direct && !HOST_PREVIEW;')){continue}
      try {
        Invoke-WebRequest -Uri 'http://127.0.0.1:43187/api/host/live-system/composer/status' -UseBasicParsing -TimeoutSec 3 -Headers @{Origin='http://127.0.0.1:43187'} | Out-Null
      }catch{
        if($_.Exception.Response -and [int]$_.Exception.Response.StatusCode -eq 401){
          $healthy=$true;break
        }
      }
    }catch{}
  }
  if(!$healthy){throw 'PINNED_HOST_HEALTH_UNVERIFIED'}
  Set-Content -Path (Join-Path $base 'LIVE_SERVED_SHA.txt') -Value $Sha -Encoding Ascii
  Write-Output ('CUTOVER_PASS sha='+$Sha+' pid='+$newProc.Id+' samePhoneURL=true')
}catch{
  $reason=$_.Exception.Message
  if($newProc){
    $newProc.Refresh()
    if(!$newProc.HasExited){Stop-Process -Id $newProc.Id -Force -ErrorAction SilentlyContinue}
  }
  Start-Sleep -Milliseconds 400
  $existing=Get-NetTCPConnection -State Listen -LocalPort 43187 -ErrorAction SilentlyContinue |
    Where-Object LocalAddress -eq '127.0.0.1' | Select-Object -First 1
  if(!$existing){
    Start-Process -FilePath $node -ArgumentList ('"'+$oldScript+'"') -WorkingDirectory $oldRoot | Out-Null
    Write-Warning ('ROLLED_BACK_TO_OLD_HOST reason='+$reason)
  }else{
    Write-Warning ('ROLLBACK_BLOCKED_PORT_OCCUPIED reason='+$reason)
  }
  throw
}
