# Windows host bootstrap: validation only unless -StartBroker explicitly supplied.
# Provision the dedicated identities and protected directories out of band.
# Does not modify ACLs, create users, store passwords, or elevate privileges.
[CmdletBinding()]
param(
  [Parameter(Mandatory)][string]$HostRoot,
  [Parameter(Mandatory)][string]$Executable,
  [Parameter(Mandatory)][string]$Store,
  [Parameter(Mandatory)][string]$AuthorityFile,
  [Parameter(Mandatory)][string]$HostSid,
  [Parameter(Mandatory)][string]$WorkerSid,
  [ValidateRange(1,1000000)][int]$RequestLimit=10000,
  [switch]$StartBroker
)
$ErrorActionPreference='Stop'
Set-StrictMode -Version Latest
if (-not $IsWindows) { throw 'Windows only; no implicit fallback' }
$hostIdentity=[System.Security.Principal.WindowsIdentity]::GetCurrent()
if ($hostIdentity.User.Value -ne $HostSid) { throw 'Run under explicitly provisioned dedicated host identity' }
if ($HostSid -eq $WorkerSid) { throw 'Host and worker identities must differ' }
$null=[System.Security.Principal.SecurityIdentifier]::new($WorkerSid)
$trusted=@('S-1-5-18','S-1-5-32-544',$HostSid)
if ($trusted -contains $WorkerSid) { throw 'Worker SID cannot be a trusted administrator/host identity' }
function Resolve-Local([string]$Path) {
  $item=Get-Item -LiteralPath $Path -Force
  if ($item.PSProvider.Name -ne 'FileSystem' -or $item.FullName.StartsWith('\\')) { throw 'Require local filesystem path' }
  $cursor=$item
  while ($null -ne $cursor) {
    if (($cursor.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) { throw "Reparse point forbidden: $($cursor.FullName)" }
    $cursor=if($cursor -is [IO.DirectoryInfo]){$cursor.Parent}else{$cursor.Directory}
  }
  return $item.FullName
}
function Assert-Contained([string]$Root,[string]$Path) {
  if (-not $Path.StartsWith($Root.TrimEnd('\')+'\',[StringComparison]::OrdinalIgnoreCase)) { throw 'Path outside protected root' }
}
function Assert-RestrictedAcl([string]$Path,[bool]$Root) {
  $acl=Get-Acl -LiteralPath $Path
  $owner=$acl.GetOwner([System.Security.Principal.SecurityIdentifier]).Value
  if ($trusted -notcontains $owner) { throw "Unexpected owner: $Path" }
  if ($Root -and -not $acl.AreAccessRulesProtected) { throw "Protected root must have a non-inherited DACL: $Path" }
  foreach ($rule in $acl.GetAccessRules($true,$true,[System.Security.Principal.SecurityIdentifier])) {
    if ($rule.AccessControlType -eq [System.Security.AccessControl.AccessControlType]::Allow -and $trusted -notcontains $rule.IdentityReference.Value) {
      throw "Unexpected allow ACE on protected path: $Path / $($rule.IdentityReference.Value)"
    }
  }
}
$root=Resolve-Local $HostRoot
$exe=Resolve-Local $Executable
$storePath=Resolve-Local $Store
$auth=Resolve-Local $AuthorityFile
foreach($path in @($exe,$storePath,$auth)){Assert-Contained $root $path}
if (-not (Test-Path -LiteralPath $storePath -PathType Container)) { throw 'Store must be a directory' }
if (-not (Test-Path -LiteralPath $exe -PathType Leaf) -or -not (Test-Path -LiteralPath $auth -PathType Leaf)) { throw 'Executable and authority must be files' }
Assert-RestrictedAcl $root $true
# Check every existing descendant rather than trusting the root ACL alone.
# Walk manually so directory reparse points cannot redirect enumeration.
$pending=[System.Collections.Generic.Stack[string]]::new()
$pending.Push($root)
while($pending.Count -gt 0){
  $dir=$pending.Pop()
  foreach($item in Get-ChildItem -LiteralPath $dir -Force){
    if (($item.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0){throw "Reparse point forbidden: $($item.FullName)"}
    Assert-RestrictedAcl $item.FullName $false
    if($item.PSIsContainer){$pending.Push($item.FullName)}
  }
}
# A preflight is a point-in-time DACL inspection, not a proof of effective
# worker access or parent-directory rename protection. An administrator must
# independently verify group membership, ancestor ACLs and worker denial.
if(-not $StartBroker){
  [pscustomobject]@{mode='preflight_only';host_sid=$HostSid;worker_sid=$WorkerSid;protected_root=$root;acl_check='passed';effective_access_verified=$false;broker_started=$false}|ConvertTo-Json -Compress
  exit 0
}
# The trusted launcher owns stdin/stdout. Do not attach arbitrary worker shells.
# The authority file is not emitted; the broker validates signed requests.
& $exe kernel $storePath ingress $auth $RequestLimit
exit $LASTEXITCODE
