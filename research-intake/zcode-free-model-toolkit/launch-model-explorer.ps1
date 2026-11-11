# Start the read-only model catalogue GUI. Does not alter provider settings.
$ErrorActionPreference = 'Stop'
$script = Join-Path $PSScriptRoot 'model_explorer.py'
$py = Get-Command py -ErrorAction SilentlyContinue
if ($py) { & $py.Source -3 $script }
else {
    $python = Get-Command python -ErrorAction Stop
    & $python.Source $script
}