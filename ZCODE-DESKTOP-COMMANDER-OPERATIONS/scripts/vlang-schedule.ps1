# VIVIM VLANG source-pinned native + four-lane observer. One Windows task.
# The existing test runner now runs in a disposable detached checkout with an
# isolated Rust binary, never replacing the live Host or using paid models.
# 12m bound leaves time for sequential native checks; one-instance mutex and
# hourly recurrence still prevent duplicate heavyweight test cycles.
[CmdletBinding()]
param([ValidateSet('status','install','run','remove')][string]$Action='status')
$ErrorActionPreference='Stop'
$TaskName='VIVIM-VLANG-Observer'
$Entry=Join-Path $PSScriptRoot 'source-pinned-native-cycle.ps1'
$Shell=Join-Path $PSHOME 'powershell.exe'
if(-not (Test-Path -LiteralPath $Entry -PathType Leaf)){throw "Missing VLANG runner: $Entry"}
if(-not (Test-Path -LiteralPath $Shell -PathType Leaf)){throw "Missing Windows PowerShell: $Shell"}
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
   $taskAction=New-ScheduledTaskAction -Execute $Shell -Argument ('-NoProfile -NonInteractive -ExecutionPolicy Bypass -File "'+$Entry+'"') -WorkingDirectory 'C:\0-BlackBoxProject-0\ARBOR'
   $settings=New-ScheduledTaskSettingsSet -MultipleInstances IgnoreNew -ExecutionTimeLimit (New-TimeSpan -Minutes 12) -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries
   $principal=New-ScheduledTaskPrincipal -UserId ([System.Security.Principal.WindowsIdentity]::GetCurrent().Name) -LogonType Interactive -RunLevel Limited
   Register-ScheduledTask -TaskName $TaskName -Description 'VIVIM isolated same-SHA native build and four-lane deterministic verifier; no provider, live Host or source effects' -Action $taskAction -Trigger $trigger -Settings $settings -Principal $principal -Force | Out-Null
   Get-State
 }
 'run' {
   if(-not (Get-ScheduledTask -TaskName $TaskName -ErrorAction SilentlyContinue)){throw "Install scheduled observer first"}
   Start-ScheduledTask -TaskName $TaskName
   Write-Output "TASK_TRIGGERED_CHECK_RECEIPT in local-logs/native-verifier/latest.json"
 }
 'remove' {
   Unregister-ScheduledTask -TaskName $TaskName -Confirm:$false -ErrorAction Stop
   Write-Output "TASK_REMOVED $TaskName"
 }
}
