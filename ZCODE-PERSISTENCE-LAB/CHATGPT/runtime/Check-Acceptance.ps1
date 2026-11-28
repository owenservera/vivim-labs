#Requires -Version 7.0
param([switch]$RequireComplete)
$ErrorActionPreference='Stop'
$t=Get-Content -Raw -LiteralPath (Join-Path $PSScriptRoot 'acceptance-tracker.json') | ConvertFrom-Json
$g=@($t.gates)
if($g.Count -ne 50 -or @($g.id | Sort-Object -Unique).Count -ne 50){throw 'Expected 50 unique gates'}
foreach($i in 1..50){if("E{0:d2}" -f $i -notin $g.id){throw "Missing gate $i"}}
foreach($x in $g){
 if($x.weight -ne 2 -or $x.status -notin @('OPEN','PARTIAL','PASS','FAIL','BLOCKED') -or $x.priority -notin @('P0','P1','P2','P3','P4')){throw "Invalid gate $($x.id)"}
 if($x.status -eq 'PASS'){
  if(@($x.evidence).Count -eq 0){throw "Missing evidence: $($x.id)"}
  foreach($e in $x.evidence){if($e.source_pin -notmatch '^[a-f0-9]{40}$' -or -not $e.detail -or -not (Test-Path -LiteralPath (Join-Path $PSScriptRoot $e.source) -PathType Leaf)){throw "Invalid evidence reference: $($x.id)"}}
 }
}
$score=2*@($g | Where-Object status -eq 'PASS').Count
Write-Output "Verified acceptance score: $score/100 (metadata validation; does not rerun native evidence)"
foreach($p in 'P0','P1','P2','P3','P4'){
 $items=@($g | Where-Object priority -eq $p);$pass=@($items | Where-Object status -eq 'PASS').Count
 Write-Output "$p : $pass/$($items.Count) gates"
}
$ledger=Get-Content -Raw -LiteralPath (Join-Path $PSScriptRoot 'ACCEPTANCE-LEDGER.md')
foreach($x in $g){$pattern='(?m)^\| '+$x.id+' \| '+[regex]::Escape($x.criterion)+' \| '+$x.status+' \|';if($ledger -notmatch $pattern){throw "Human ledger differs: $($x.id)"}}
if($RequireComplete -and $score -ne 100){throw "Acceptance incomplete: $score/100"}
