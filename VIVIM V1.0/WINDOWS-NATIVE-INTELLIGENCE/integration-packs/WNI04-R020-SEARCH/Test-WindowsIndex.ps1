param([Parameter(Mandatory)][string]$FixtureParent)
. (Join-Path $PSScriptRoot 'WindowsIndex.ps1')
$parent=Get-WniLocalPath $FixtureParent $true
$d=Join-Path $parent.FullName ('wni-index-fixture-'+[guid]::NewGuid().ToString('N'))
[void](New-Item -ItemType Directory -Path $d)
try {
    $name="wni-O'Brien.txt";$file=Join-Path $d $name
    [IO.File]::WriteAllText($file,'owned-fixture',[Text.UTF8Encoding]::new($false))
    $q=New-WniSearchQuery $d $name 2
    if (!$q.Contains("O''Brien.txt") -or !$q.StartsWith('SELECT TOP 2 ')){throw 'QUERY_ESCAPE'}
    $hit=Get-WniRelativeFile $d $file $name
    if ($hit.size -ne 13 -or $hit.relative -ne $name){throw 'FILE_PROJECTION'}
    if ($null -ne (Get-WniRelativeFile $d ($d+'-outside\'+$name) $name)){throw 'SIBLING_ESCAPE'}
    $refused=$false;try{New-WniSearchQuery $d '..\x' 2|Out-Null}catch{$refused=$true}
    if(!$refused){throw 'TRAVERSAL_NOT_REFUSED'}
    $refused=$false;try{New-WniSearchQuery $d '.env' 2|Out-Null}catch{$refused=$true}
    if(!$refused){throw 'SENSITIVE_NOT_REFUSED'}
    $r=Invoke-WniIndexedSearch $d $name 2
    if($r.status -notin @('ok','unavailable') -or $r.complete -ne $false -or @($r.rows).Count -gt 2){throw 'INDEX_CONTRACT'}
    foreach($row in $r.rows){if($row.relative -ne $name -or $row.size -ne 13){throw 'INDEX_RESULT'}}
    $p=Get-WniShellProperty $d $name
    if($p.status -eq 'ok' -and $p.size -ne 13){throw 'PROPERTY_RESULT'}
    [pscustomobject]@{checks=7;indexedSearch=$r.status;indexedRows=@($r.rows).Count;indexCoverageComplete=$r.complete;shellProperty=$p.status;sizeVerified=($p.status -eq 'ok' -and $p.size -eq 13);rawPathsPublished=$false}|ConvertTo-Json -Compress
} finally {
    # Delete only the uniquely created owned fixture, never its parent.
    Remove-Item -LiteralPath $d -Recurse -Force
}
