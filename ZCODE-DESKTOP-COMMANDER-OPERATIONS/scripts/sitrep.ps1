# Read-only ZCode/Commander status; no credentials or model prompt calls.
$ErrorActionPreference = 'Continue'
$homeDir = [Environment]::GetFolderPath('UserProfile')
$base = Join-Path $homeDir '.zcode'
$repo = 'C:\0-BlackBoxProject-0\vivim-labs'
Write-Output "ZCODE COMMANDER SITREP R001"
Write-Output ("timestamp_utc=" + [DateTime]::UtcNow.ToString('o'))
Write-Output ("computer=" + $env:COMPUTERNAME)
try { $v = (& zcode --version 2>$null | Select-Object -First 1); Write-Output ("zcode_version=" + $v) } catch { Write-Output "zcode_version=UNAVAILABLE" }
$agentsDir = Join-Path $base 'agents'
$agentFiles = @(Get-ChildItem $agentsDir -File -Filter '*.md' -ErrorAction SilentlyContinue | Where-Object { $_.Name -notmatch '\.bak' })
Write-Output ("agent_definitions=" + $agentFiles.Count)
$comms = Join-Path $base 'comms'
foreach ($name in @('bus.jsonl','delivered.jsonl','receipts.jsonl','projects.json')) {
  $f = Join-Path $comms $name
  if(Test-Path $f) { $item=Get-Item $f; Write-Output ("bus_file=" + $name + " bytes=" + $item.Length + " modified_utc=" + $item.LastWriteTimeUtc.ToString('o')) }
  else { Write-Output ("bus_file=" + $name + " MISSING") }
}
$configPath = Join-Path $base 'v2\provider_config.json'
if(Test-Path $configPath) {
  try {
    $cfg = (Get-Content $configPath -Raw | ConvertFrom-Json).config
    Write-Output ("provider_rules=" + @($cfg.providerConfigRules.providerRules).Count)
    Write-Output ("model_rules=" + @($cfg.modelConfigRules.providerModelRules).Count)
    foreach ($id in @('opencode-zen-chat-2','new-provider','minimax-m-plan','account:zai-start-plan')) {
      $record = $cfg.providerConfigRules.providerRules | Where-Object { $_.providerId -eq $id } | Select-Object -First 1
      $count = @($cfg.modelConfigRules.providerModelRules | Where-Object { $_.providerId -eq $id }).Count
      $enabled = if ($null -eq $record.enabled) { 'account_or_unspecified' } else { [string]$record.enabled }
      Write-Output ("provider=" + $id + " enabled=" + $enabled + " model_rules=" + $count)
    }
  } catch { Write-Output 'provider_inventory=PARSE_ERROR' }
}
try { $pr = @(Get-CimInstance Win32_Process -ErrorAction Stop | Where-Object { $_.Name -match '^zcode(.exe)?$' -or $_.CommandLine -match '(?i)zcode[^\\]*app-server' }); Write-Output ("zcode_process_matches=" + $pr.Count) }
catch { Write-Output 'zcode_process_matches=UNKNOWN' }
try { $head = (& git -C $repo rev-parse --short HEAD 2>$null | Select-Object -First 1); Write-Output ("labs_head=" + $head) } catch { Write-Output 'labs_head=UNKNOWN' }
Write-Output 'SITREP_COMPLETE'
