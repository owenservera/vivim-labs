param([switch]$Deep,[switch]$Team,[switch]$V4)
# Incremental, secret-safe ZCode operations SITREP.
$ErrorActionPreference='Continue'
$scriptHome=Split-Path -Parent $MyInvocation.MyCommand.Path
Write-Output '=== BASE SITREP ==='
& (Join-Path $scriptHome 'sitrep.ps1')
Write-Output '=== PERSISTED SESSION / WORKFLOW COUNTS (READ-ONLY SQLITE) ==='
$db=Join-Path ([Environment]::GetFolderPath('UserProfile')) '.zcode\cli\db\db.sqlite'
$sqlite='C:\Users\VIVIM.inc\Tools\disksteward\sqlite\sqlite3.exe'
if((Test-Path $sqlite) -and (Test-Path $db)){
  $query = "SELECT 'stored_sessions=' || COUNT(*) FROM session UNION ALL SELECT 'child_sessions=' || COUNT(*) FROM session WHERE parent_id IS NOT NULL UNION ALL SELECT 'task_links=' || COUNT(*) FROM session_task_link UNION ALL SELECT 'workflow_runs=' || COUNT(*) FROM workflow_run UNION ALL SELECT 'workflow_definitions=' || COUNT(*) FROM workflow_definition;"
  & $sqlite -readonly $db $query
  Write-Output ('sqlite_exit_code='+$LASTEXITCODE)
} else {Write-Output 'sqlite_database_or_executable=MISSING'}
Write-Output '=== WORKFLOW FILE CATALOG ==='
$wfRoot='C:\0-BlackBoxProject-0\ARBOR\.zcode\workflows'
if(Test-Path $wfRoot){
  $files=@(Get-ChildItem $wfRoot -File -Filter '*.dwf.ts' -ErrorAction SilentlyContinue)
  Write-Output ('arbor_saved_workflow_files='+$files.Count)
  $files | Sort-Object Name | ForEach-Object {Write-Output ('workflow_name='+($_.Name -replace '\.dwf\.ts$',''))}
}else{Write-Output 'arbor_workflow_directory=MISSING'}
Write-Output '=== RUNTIME PORT / NATIVE PROTOCOL ==='
if($Deep){
  $node='C:\Program Files\nodejs\node.exe'
  $probe=Join-Path $scriptHome 'probe-protocol.ts'
  if((Test-Path $node) -and (Test-Path $probe)){
    & $node $probe
    Write-Output ('protocol_probe_exit_code='+$LASTEXITCODE)
  }else { Write-Output 'protocol_probe=UNAVAILABLE'}
}else{ Write-Output 'protocol_probe=SKIPPED (pass -Deep for read-only app-server test)' }
if($V4){
  Write-Output '=== READ-ONLY V4 SUBSCRIPTION CHECK ==='
  & 'C:\Program Files\nodejs\node.exe' (Join-Path $scriptHome 'probe-v4-subscribe.ts')
  Write-Output ('v4_probe_exit_code='+$LASTEXITCODE)
}
if($Team){ & (Join-Path $scriptHome 'hq-team-sitrep.ps1') }
Write-Output 'SITREP_R002_COMPLETE'
