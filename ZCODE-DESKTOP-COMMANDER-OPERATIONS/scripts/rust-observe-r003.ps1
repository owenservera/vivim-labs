param([switch]$WithHistoricalParent)
$ErrorActionPreference = "Stop"
$crate = Join-Path $PSScriptRoot "..\native\zcode-observer"
Push-Location $crate
try {
    & cargo test --offline
    if ($LASTEXITCODE -ne 0) { throw "RUST_TEST_FAIL" }

    if ($WithHistoricalParent) {
        $db = Join-Path $HOME ".zcode\cli\db\db.sqlite"
        if (-not (Test-Path -LiteralPath $db)) { throw "HISTORICAL_DB_MISSING" }
        $parent = & sqlite3.exe -readonly $db 'SELECT parent_session_id FROM session_task_link WHERE parent_session_id IS NOT NULL GROUP BY parent_session_id ORDER BY COUNT(*) DESC LIMIT 1;'
        if ($LASTEXITCODE -ne 0 -or [string]::IsNullOrWhiteSpace($parent)) {
            throw "HISTORICAL_PARENT_NOT_FOUND"
        }
        $env:ZCODE_OBSERVER_PARENT_SESSION = $parent.Trim()
    }

    & cargo run --offline
    if ($LASTEXITCODE -ne 0) { throw "RUST_OBSERVER_FAIL" }
    Write-Output "R003_RUST_OBSERVER_COMPLETE"
} finally {
    Remove-Item Env:\ZCODE_OBSERVER_PARENT_SESSION -ErrorAction SilentlyContinue
    Pop-Location
}
