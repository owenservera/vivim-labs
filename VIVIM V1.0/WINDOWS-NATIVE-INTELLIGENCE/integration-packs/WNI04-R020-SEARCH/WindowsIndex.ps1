# Labs-only COM adapter. The caller MUST bind root to the current approved Host scope.
# This is not a security boundary or a public terminal action.
Set-StrictMode -Version Latest
$ErrorActionPreference='Stop'
function Test-WniComponent([string]$Name) {
    if (!$Name -or $Name.Length -gt 255 -or $Name -match '[\x00-\x1f\x7f\\/:*?"<>|%#]' -or $Name -match '[. ]$' -or $Name -in @('.','..')) {return $false}
    if ($Name -match '^(?i:CON|PRN|AUX|NUL|COM[1-9]|LPT[1-9])(?:\.|$)') {return $false}
    if ($Name -match '^(?i:\.git|\.ssh|\.aws|\.azure|\.gnupg|\.env(?:\..*)?|credentials(?:\..*)?|cookies(?:\..*)?|login data|local state|secrets(?:\..*)?|user data)$') {return $false}
    return $true
}
function Get-WniLocalPath([string]$Path,[bool]$Directory) {
    if (!$IsWindows -or $Path.Length -gt 1024 -or $Path -notmatch '^[A-Za-z]:\\[^\\]') {throw 'INVALID_LOCAL_PATH'}
    foreach ($part in $Path.Substring(3).Split('\')) {if (!(Test-WniComponent $part)){throw 'INVALID_COMPONENT'}}
    $full=[IO.Path]::GetFullPath($Path)
    if (![string]::Equals($full,$Path,[StringComparison]::OrdinalIgnoreCase)){throw 'PATH_ALIAS'}
    if ([IO.DriveInfo]::new($full.Substring(0,3)).DriveType -ne [IO.DriveType]::Fixed){throw 'NOT_FIXED_DRIVE'}
    $walk=$full
    while ($walk) {
        $i=Get-Item -LiteralPath $walk -Force
        if (($i.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0){throw 'REPARSE_POINT'}
        $walk=[IO.Path]::GetDirectoryName($walk)
    }
    $item=Get-Item -LiteralPath $full -Force
    if ($item.PSIsContainer -ne $Directory){throw 'WRONG_ITEM_KIND'}
    return $item
}
function Get-WniRelativeFile([string]$Root,[string]$Path,[string]$Name) {
    if (!$Path.StartsWith($Root+'\',[StringComparison]::OrdinalIgnoreCase)){return $null}
    if (![string]::Equals([IO.Path]::GetFileName($Path),$Name,[StringComparison]::OrdinalIgnoreCase)){return $null}
    try {
        $f=Get-WniLocalPath $Path $false
        $relative=[IO.Path]::GetRelativePath($Root,$f.FullName)
        if ([IO.Path]::IsPathRooted($relative) -or $relative.StartsWith('..\')){return $null}
        return [pscustomobject]@{relative=$relative;size=[long]$f.Length;modifiedUtc=$f.LastWriteTimeUtc.ToString('o')}
    } catch {return $null}
}
function New-WniSearchQuery([string]$Root,[string]$Name,[int]$Limit) {
    if (!(Test-WniComponent $Name) -or $Limit -lt 1 -or $Limit -gt 200){throw 'INVALID_QUERY'}
    $folder=Get-WniLocalPath $Root $true
    $s=('file:'+$folder.FullName.Replace('\','/')).Replace("'","''")
    $n=$Name.Replace("'","''")
    return "SELECT TOP $Limit System.ItemPathDisplay FROM SYSTEMINDEX WHERE SCOPE='$s' AND System.FileName='$n' ORDER BY System.DateModified DESC"
}
function Invoke-WniIndexedSearch([string]$Root,[string]$Name,[int]$Limit=20) {
    $sql=New-WniSearchQuery $Root $Name $Limit
    $rootItem=Get-WniLocalPath $Root $true
    $result=[ordered]@{schema='vivim.windows.indexed-search/1';status='unavailable';coverage='index-only';complete=$false;rows=@();observed=0;discarded=0;atLimit=$false}
    $conn=$null;$rs=$null
    try {
        $conn=New-Object -ComObject ADODB.Connection
        $conn.ConnectionTimeout=3;$conn.CommandTimeout=5
        $conn.Open("Provider=Search.CollatorDSO.1;Extended Properties='Application=Windows';")
        $rs=$conn.Execute($sql)
        $rows=[Collections.Generic.List[object]]::new()
        $seen=[Collections.Generic.HashSet[string]]::new([StringComparer]::OrdinalIgnoreCase)
        while (!$rs.EOF -and $result.observed -lt $Limit) {
            $result.observed++
            $raw=$rs.Fields.Item(0).Value
            $hit=$null
            if ($raw -is [string]){$hit=Get-WniRelativeFile $rootItem.FullName $raw $Name}
            if ($null -ne $hit -and $seen.Add($hit.relative)){$rows.Add($hit)}else{$result.discarded++}
            $rs.MoveNext()
        }
        $result.atLimit=$result.observed -ge $Limit
        # A successful query is not proof that the entire folder was indexed.
        $result.rows=@($rows.ToArray());$result.status='ok'
    } catch {
        # Never echo provider exception text (may contain paths or SQL).
        $result.status='unavailable';$result.rows=@()
    } finally {
        if ($null -ne $rs){try{$rs.Close()}catch{};if([Runtime.InteropServices.Marshal]::IsComObject($rs)){[void][Runtime.InteropServices.Marshal]::FinalReleaseComObject($rs)}}
        if ($null -ne $conn){try{$conn.Close()}catch{};if([Runtime.InteropServices.Marshal]::IsComObject($conn)){[void][Runtime.InteropServices.Marshal]::FinalReleaseComObject($conn)}}
    }
    return [pscustomobject]$result
}
function Get-WniShellProperty([string]$Root,[string]$Relative) {
    $rootItem=Get-WniLocalPath $Root $true
    if ([IO.Path]::IsPathRooted($Relative)){throw 'INVALID_RELATIVE'}
    $candidate=Join-Path $rootItem.FullName $Relative
    $f=Get-WniRelativeFile $rootItem.FullName $candidate ([IO.Path]::GetFileName($candidate))
    if ($null -eq $f){throw 'NOT_ADMITTED'}
    $item=Get-WniLocalPath $candidate $false
    $result=[ordered]@{schema='vivim.windows.shell-property/1';status='unavailable';size=$null;itemType=$null}
    $shell=$null;$folder=$null;$entry=$null
    try{
        $shell=New-Object -ComObject Shell.Application
        $folder=$shell.NameSpace($item.DirectoryName)
        if($null -eq $folder){return [pscustomobject]$result}
        $entry=$folder.ParseName($item.Name)
        if($null -eq $entry){return [pscustomobject]$result}
        $size=$entry.ExtendedProperty('System.Size')
        $type=$entry.ExtendedProperty('System.ItemType')
        if($null -ne $size -and $size -isnot [DBNull]){
            $n=[long]$size
            if($n -eq $item.Length -and $n -ge 0){$result.size=$n}
        }
        if($type -is [string] -and $type -match '^\.[A-Za-z0-9_-]{1,16}$'){$result.itemType=$type}
        if($null -ne $result.size){$result.status='ok'}
        # No author, title, content, thumbnail, tags or arbitrary properties.
    }catch{$result.status='unavailable';$result.size=$null;$result.itemType=$null}
    finally{
        foreach($o in @($entry,$folder,$shell)){if($null -ne $o -and [Runtime.InteropServices.Marshal]::IsComObject($o)){[void][Runtime.InteropServices.Marshal]::FinalReleaseComObject($o)}}
    }
    return [pscustomobject]$result
}
