#Requires -Version 7.0
[CmdletBinding()]
param([Parameter(Mandatory)][string]$RunRoot)
$ErrorActionPreference = 'Stop'
$root = [System.IO.Path]::GetFullPath($RunRoot)
if (Test-Path -LiteralPath $root) { throw "RunRoot must not exist: $root" }
$source = [System.IO.Path]::GetFullPath($PSScriptRoot)
if ($root.StartsWith($source + [IO.Path]::DirectorySeparatorChar, [StringComparison]::OrdinalIgnoreCase)) { throw 'Use a separate run root outside this scaffold.' }
New-Item -ItemType Directory -Path $root | Out-Null
foreach ($id in @('01','02','03','04')) {
    $lane = Join-Path $root $id
    New-Item -ItemType Directory -Path $lane | Out-Null
    if (@(Get-ChildItem -LiteralPath $lane -Force).Count -ne 0) { throw "Lane not empty: $id" }
}
$control = Join-Path $root '_controller'
New-Item -ItemType Directory -Path $control | Out-Null
foreach ($dir in @('events','receipts','checkpoints','transcripts','faults')) {
    New-Item -ItemType Directory -Path (Join-Path $control $dir) | Out-Null
}
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'experiment.json') -Destination $control
[ordered]@{schema='vivim.lab.run/1';run_id=[guid]::NewGuid().ToString('N');created_utc=[DateTime]::UtcNow.ToString('o');status='INITIALIZED_NOT_LAUNCHED';lanes=@('01','02','03','04')} | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath (Join-Path $control 'run.json') -Encoding utf8NoBOM
Write-Output "Initialized four empty lanes at $root. No agents started."
