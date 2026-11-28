# Prepare a manifest pinned to the actual canonical tracker bytes.
# No build, process execution, provider request, or store creation.
[CmdletBinding()]
param(
 [Parameter(Mandatory)][string]$TrackerPath,
 [Parameter(Mandatory)][string]$OutputPath,
 [ValidatePattern('^[A-Za-z0-9_-]{1,60}$')][string]$GoalId='genesis-0001',
 [long]$InputValue=42,
 [ValidateRange(4,100)][int]$ClaimBudget=8
)
$ErrorActionPreference='Stop'
Set-StrictMode -Version Latest
$tracker=Get-Item -LiteralPath $TrackerPath
if($tracker.PSIsContainer){throw 'Tracker must be a file'}
$raw=[IO.File]::ReadAllBytes($tracker.FullName)
$t=[Text.Encoding]::UTF8.GetString($raw)|ConvertFrom-Json
if(@($t.gates).Count -ne 50 -or @($t.routing_workstream.gates).Count -ne 10){throw 'Unexpected tracker register'}
if($InputValue -gt ([long]::MaxValue-2)){throw 'Input would overflow the two-step bootstrap'}
$hash=[Convert]::ToHexString([Security.Cryptography.SHA256]::HashData($raw)).ToLowerInvariant()
$manifest=[ordered]@{id=$GoalId;objective='Qualify native objective persistence, dependent execution and independent artifact verification';tracker_digest=$hash;input=$InputValue;claim_budget=$ClaimBudget}
$dest=[IO.Path]::GetFullPath($OutputPath)
if(Test-Path -LiteralPath $dest){throw 'Refuse to overwrite an existing launch manifest'}
$bytes=[Text.Encoding]::UTF8.GetBytes(($manifest|ConvertTo-Json -Depth 10))
$stream=[IO.File]::Open($dest,[IO.FileMode]::CreateNew,[IO.FileAccess]::Write,[IO.FileShare]::None)
try{$stream.Write($bytes,0,$bytes.Length);$stream.Flush($true)}finally{$stream.Dispose()}
[pscustomobject]@{manifest=$dest;tracker_sha256=$hash;execution='not_started'}|ConvertTo-Json -Compress
