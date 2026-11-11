# Read-only diagnostics. No secrets printed, no config modifications.
[CmdletBinding()]
param([string]$BaseUrl = 'http://127.0.0.1:3001', [int]$TimeoutSec = 8)
$ErrorActionPreference = 'Stop'
Write-Host 'FreeLLMAPI / ZCode — read-only doctor'
try {
  $u = [uri]$BaseUrl
  if ($u.Scheme -ne 'http' -or $u.Host -notin @('127.0.0.1','localhost','::1')) {
    throw 'Only loopback HTTP endpoints are accepted by this diagnostic.'
  }
  $node = Get-Command node -ErrorAction SilentlyContinue
  $npx = Get-Command npx -ErrorAction SilentlyContinue
  Write-Host ('Node present: ' + [bool]$node + '; npx present: ' + [bool]$npx)
  if ($node) { node --version }
  $config = if ($env:ZCODE_PERSONAL_PROVIDER_CONFIG_FILE) {
    $env:ZCODE_PERSONAL_PROVIDER_CONFIG_FILE
  } else { Join-Path $HOME '.zcode/v2/provider_config.json' }
  Write-Host ('ZCode registry exists: ' + (Test-Path -LiteralPath $config))
  Write-Host ('Registry location: ' + $config)
  Write-Host ('Unified key in this shell: ' + [bool]$env:FREELLMAPI_API_KEY)
  $headers = @{}
  if ($env:FREELLMAPI_API_KEY) { $headers['Authorization'] = 'Bearer ' + $env:FREELLMAPI_API_KEY }
  $models = Invoke-RestMethod -Method Get -Uri ($BaseUrl.TrimEnd('/') + '/v1/models') -Headers $headers -TimeoutSec $TimeoutSec
  if ($models.object -ne 'list' -or $null -eq $models.data) { throw 'Unexpected /v1/models response shape.' }
  $ids = @($models.data | ForEach-Object { $_.id } | Where-Object { $_ -is [string] -and $_.Length -gt 0 })
  Write-Host ('Model IDs returned: ' + $ids.Count)
  Write-Host ('Auto model advertised: ' + [bool]($ids -contains 'auto'))
  Write-Host 'PASS: endpoint reachable and models response parsed.'
  Write-Host 'NOT CHECKED: completion, tool calling, streaming, billing, ZCode runtime integration.'
} catch {
  Write-Error ('FAIL: ' + $_.Exception.Message)
  exit 1
}
