# Read-only HQ team snapshot. Do not pass --write-state or seat wake.
$ErrorActionPreference='Continue'
$bin='C:\Users\VIVIM.inc\.bun\bin\bun.exe'
$repo='C:\0-BlackBoxProject-0\repos_hq'
$entry=Join-Path $repo 'hq\bin\hq.mjs'
if(!(Test-Path $bin) -or !(Test-Path $entry)){Write-Output 'HQ_TEAM=UNAVAILABLE';exit 2}
Push-Location $repo
try {
  $lines=@(& $bin $entry team --json 2>&1)
  $resultCode=$LASTEXITCODE
  Write-Output ('HQ_TEAM_COMMAND_EXIT='+$resultCode)
  if($resultCode -ne 0){Write-Output 'HQ_TEAM=FAILED_NONZERO';exit 1}
  try {
    $j=($lines|Out-String)|ConvertFrom-Json -ErrorAction Stop
    $names=$j.PSObject.Properties.Name
    Write-Output ('HQ_TEAM_KEYS='+($names -join ','))
    foreach($key in @('agents','seats','tasks','defects','nextMoves')){
      if($names -contains $key){Write-Output ('HQ_TEAM_'+$key+'_COUNT='+@($j.$key).Count)}
    }
    Write-Output 'HQ_TEAM_SITREP_COMPLETE'
  } catch {Write-Output 'HQ_TEAM=INVALID_JSON';exit 1}
} finally {Pop-Location}
