# VIVIM VLANG read-only incremental observer. One Windows scheduled task,
# no background LLM spending, no Host/browser effects, no source edits.
[CmdletBinding()]
param([ValidateSet('status','install','run','remove')][string]$Action='status')
$ErrorActionPreference='Stop'
$TaskName='VIVIM-VLANG-Observer'
$Entry=Join-Path $PSScriptRoot 'vlang-parallel-cycle.ts'
$Node='C:\Program Files\nodejs\node.exe'
if(-not (Test-Path -LiteralPath $Entry -PathType Leaf)){throw "Missing VLANG runner: $Entry"}
if(-not (Test-Path -LiteralPath $Node -PathType Leaf)){throw "Missing Node 24+ at $Node"}
function Get-State {
  $task=Get-ScheduledTask -TaskName $TaskName -ErrorAction SilentlyContinue
  if(-not $task){Write-Output "TASK_ABSENT $TaskName";return}
  $info=Get-ScheduledTaskInfo -TaskName $TaskName -ErrorAction SilentlyContinue
  Write-Output ("TASK="+$TaskName+" STATUS="+$task.State+" LAST="+$info.LastRunTime+
    " RESULT="+$info.LastTaskResult+" NEXT="+$info.NextRunTime)
  Write-Output ("PROGRAM="+$task.Actions.Execute+" ARGUMENTS="+$task.Actions.Arguments)
}
switch($Action){
 'status' {Get-State}
 'install' {
   $trigger=New-ScheduledTaskTrigger -Once -At (Get-Date).AddHours(1) -RepetitionInterval (New-TimeSpan -Hours 1) -RepetitionDuration (New-TimeSpan -Days 365)
   $taskAction=New-ScheduledTaskAction -Execute $Node -Argument ('--experimental-strip-types "'+$Entry+'" --once') -WorkingDirectory 'C:\0-BlackBoxProject-0\ARBOR'
   $settings=New-ScheduledTaskSettingsSet -MultipleInstances IgnoreNew -ExecutionTimeLimit (New-TimeSpan -Minutes 4) -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries
   $principal=New-ScheduledTaskPrincipal -UserId ([System.Security.Principal.WindowsIdentity]::GetCurrent().Name) -LogonType Interactive -RunLevel Limited
   Register-ScheduledTask -TaskName $TaskName -Description 'VIVIM exact-SHA four-lane VLANG deterministic regression observer; no AI requests, no source changes' -Action $taskAction -Trigger $trigger -Settings $settings -Principal $principal -Force | Out-Null
   Get-State
 }
 'run' {
   if(-not (Get-ScheduledTask -TaskName $TaskName -ErrorAction SilentlyContinue)){throw "Install scheduled observer first"}
   Start-ScheduledTask -TaskName $TaskName
   Write-Output "TASK_TRIGGERED_CHECK_RECEIPT in local-logs/vlang-iterations/latest.json"
 }
 'remove' {
   Unregister-ScheduledTask -TaskName $TaskName -Confirm:$false -ErrorAction Stop
   Write-Output "TASK_REMOVED $TaskName"
 }
}
