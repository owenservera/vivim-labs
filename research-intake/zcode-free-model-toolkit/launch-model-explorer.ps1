# Start the read-only model catalogue GUI. Does not alter provider settings.
$ErrorActionPreference = 'Stop'
$python = Get-Command python -ErrorAction Stop
& $python.Source (Join-Path $PSScriptRoot 'model_explorer.py')