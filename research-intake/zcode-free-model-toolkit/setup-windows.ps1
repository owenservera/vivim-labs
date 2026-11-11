# One-time setup for local Windows user. No admin rights or GitHub credentials.
param([switch]$SkipTask)
$ErrorActionPreference = 'Stop'
if ($env:OS -ne 'Windows_NT') { throw 'Windows only' }
$root = Join-Path $env:LOCALAPPDATA 'ZFreeZCode'
New-Item -ItemType Directory -Path $root -Force | Out-Null

function Save-ZFreeSecret([string]$name, [Security.SecureString]$value) {
    $path = Join-Path $root ($name + '.dpapi')
    $encrypted = $value | ConvertFrom-SecureString
    [IO.File]::WriteAllText($path, $encrypted)
}
function Read-ZFreeSecret([string]$name) {
    $path = Join-Path $root ($name + '.dpapi')
    $secure = (Get-Content -LiteralPath $path -Raw) | ConvertTo-SecureString
    $ptr = [Runtime.InteropServices.Marshal]::SecureStringToBSTR($secure)
    try { return [Runtime.InteropServices.Marshal]::PtrToStringBSTR($ptr) }
    finally { [Runtime.InteropServices.Marshal]::ZeroFreeBSTR($ptr) }
}
$tokenFile = Join-Path $root 'local-token.dpapi'
if (-not (Test-Path -LiteralPath $tokenFile)) {
    $bytes = New-Object byte[] 32
    $rng = [Security.Cryptography.RandomNumberGenerator]::Create()
    try { $rng.GetBytes($bytes) }
    finally { $rng.Dispose() }
    $token = [BitConverter]::ToString($bytes).Replace('-', '').ToLowerInvariant()
    Save-ZFreeSecret 'local-token' (ConvertTo-SecureString $token -AsPlainText -Force)
}
else { $token = Read-ZFreeSecret 'local-token' }

$orFile = Join-Path $root 'openrouter.dpapi'
if (-not (Test-Path -LiteralPath $orFile)) {
    $secret = Read-Host -Prompt 'OpenRouter API key (input hidden)' -AsSecureString
    if ($secret.Length -gt 0) { Save-ZFreeSecret 'openrouter' $secret }
}
$kiloFile = Join-Path $root 'kilo.dpapi'
if (-not (Test-Path -LiteralPath $kiloFile)) {
    Write-Host 'Kilo Gateway free models support anonymous requests; optional API key is not required.'
    $kiloSecret = Read-Host -Prompt 'Kilo Gateway API key (optional, input hidden)' -AsSecureString
    if ($kiloSecret.Length -gt 0) { Save-ZFreeSecret 'kilo' $kiloSecret }
}
$nimChoice = Read-Host -Prompt 'Enable NVIDIA developer prototyping (NOT verified permanently free)? [y/N]'
if ($nimChoice -match '^(y|yes)$') {
    $secret = Read-Host -Prompt 'NVIDIA API key (input hidden)' -AsSecureString
    if ($secret.Length -eq 0) { throw 'NVIDIA key cannot be empty when enabling NIM trial' }
    Save-ZFreeSecret 'nvidia' $secret
    [IO.File]::WriteAllText((Join-Path $root 'nim-trial-opt-in.flag'), 'explicit-user-opt-in')
}
else {
    # A later explicit No disables the NIM prototype route.
    Remove-Item -LiteralPath (Join-Path $root 'nim-trial-opt-in.flag') -ErrorAction SilentlyContinue
}
Write-Host ''
Write-Host 'ZCode custom provider: OpenAI Chat Completions'
Write-Host 'Base URL: http://127.0.0.1:8765/v1'
Write-Host 'Model: free-auto'
Write-Host ('Local-only API key (paste into ZCode; do not publish): ' + $token)

if (-not $SkipTask) {
    $start = Join-Path $PSScriptRoot 'start-windows.ps1'
    $action = New-ScheduledTaskAction -Execute 'powershell.exe' -Argument ('-NoProfile -ExecutionPolicy Bypass -File "' + $start + '"')
    $identity = [Security.Principal.WindowsIdentity]::GetCurrent().Name
    $trigger = New-ScheduledTaskTrigger -AtLogOn -User $identity
    $principal = New-ScheduledTaskPrincipal -UserId $identity -LogonType Interactive -RunLevel Limited
    Register-ScheduledTask -TaskName 'ZFree ZCode Gateway' -Action $action -Trigger $trigger -Principal $principal -Force | Out-Null
    Start-ScheduledTask -TaskName 'ZFree ZCode Gateway'
    Write-Host 'Login-started ZFree Windows task registered and started.'
}
else { Write-Host 'Task skipped. Run start-windows.ps1 manually.' }
Write-Host 'Run python -m unittest discover -s . -p test_*.py -v before relying on the gateway.'
