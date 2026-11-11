# Runs ZFree in the signed-in Windows user's session; no plaintext credentials on disk.
$ErrorActionPreference = 'Stop'
if ($env:OS -ne 'Windows_NT') { throw 'Windows only' }
$root = Join-Path $env:LOCALAPPDATA 'ZFreeZCode'
function Read-ZFreeSecret([string]$name) {
    $path = Join-Path $root ($name + '.dpapi')
    if (-not (Test-Path -LiteralPath $path)) { return $null }
    $secure = (Get-Content -LiteralPath $path -Raw) | ConvertTo-SecureString
    $ptr = [Runtime.InteropServices.Marshal]::SecureStringToBSTR($secure)
    try { return [Runtime.InteropServices.Marshal]::PtrToStringBSTR($ptr) }
    finally { [Runtime.InteropServices.Marshal]::ZeroFreeBSTR($ptr) }
}
$env:ZFREE_LOCAL_TOKEN = Read-ZFreeSecret 'local-token'
$env:OPENROUTER_API_KEY = Read-ZFreeSecret 'openrouter'
$env:NVIDIA_API_KEY = Read-ZFreeSecret 'nvidia'
$env:KILO_API_KEY = Read-ZFreeSecret 'kilo'
$env:ZFREE_ALLOW_KILO = '1'
$env:ZFREE_ALLOW_NIM_TRIAL = if (Test-Path -LiteralPath (Join-Path $root 'nim-trial-opt-in.flag')) { '1' } else { '0' }
$env:ZFREE_HOME = Join-Path $root 'state'
if (-not $env:ZFREE_LOCAL_TOKEN) { throw 'Run setup-windows.ps1 first' }
$script = Join-Path $PSScriptRoot 'zfree.py'
$py = Get-Command py -ErrorAction SilentlyContinue
if ($py) { & $py.Source -3 $script serve }
else {
    $python = Get-Command python -ErrorAction Stop
    & $python.Source $script serve
}
if ($LASTEXITCODE -ne 0) { throw ('ZFree exited with code ' + $LASTEXITCODE) }
