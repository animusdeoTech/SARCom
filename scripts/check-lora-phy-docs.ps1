# scripts/check-lora-phy-docs.ps1
#
# Advisory preflight check for lora-phy doc availability. Always exits 0;
# this is not a CI gate. See resources/docs/lora-phy-preflight.md.

$ErrorActionPreference = 'Continue'

$repoRoot     = Split-Path -Parent $PSScriptRoot
$cargoLock    = Join-Path $repoRoot 'Cargo.lock'
$rustdocIndex = Join-Path $repoRoot 'target\doc\lora_phy\index.html'
$preflightDoc = Join-Path $repoRoot 'resources\docs\lora-phy-preflight.md'

$cargoLockExists = Test-Path -LiteralPath $cargoLock
$pinned          = $false
$pinnedVersion   = $null

if ($cargoLockExists) {
    $lines = Get-Content -LiteralPath $cargoLock
    for ($i = 0; $i -lt $lines.Count; $i++) {
        if ($lines[$i] -eq 'name = "lora-phy"') {
            $pinned = $true
            if (($i + 1) -lt $lines.Count -and $lines[$i + 1] -match '^version = "(.+)"$') {
                $pinnedVersion = $matches[1]
            }
            break
        }
    }
}

$rustdocExists = Test-Path -LiteralPath $rustdocIndex

Write-Output 'lora-phy preflight status'
Write-Output '-------------------------'
if ($cargoLockExists) {
    Write-Output ("Cargo.lock:        found at {0}" -f $cargoLock)
} else {
    Write-Output 'Cargo.lock:        not found at workspace root'
}
if ($pinned) {
    if ($pinnedVersion) {
        Write-Output ("lora-phy pinned:   yes (version {0})" -f $pinnedVersion)
    } else {
        Write-Output 'lora-phy pinned:   yes (version unknown)'
    }
} else {
    Write-Output 'lora-phy pinned:   no'
}
if ($rustdocExists) {
    Write-Output ("Local rustdoc:     {0}" -f $rustdocIndex)
} else {
    Write-Output 'Local rustdoc:     missing'
}

Write-Output ''
Write-Output 'Recommended next action'
Write-Output '-----------------------'
if ($pinned -and $rustdocExists) {
    Write-Output 'Use local rustdoc first. Open target/doc/lora_phy/index.html.'
} elseif ($pinned -and -not $rustdocExists) {
    Write-Output 'Run: cargo doc -p lora-phy --locked'
    Write-Output 'Then re-run this script and use local rustdoc.'
} else {
    Write-Output 'lora-phy is not pinned. Use the github MCP against lora-rs/lora-rs.'
    Write-Output 'Do NOT write production radio code from memory.'
    if (Test-Path -LiteralPath $preflightDoc) {
        Write-Output ("See {0}." -f $preflightDoc)
    } else {
        Write-Output 'See resources/docs/lora-phy-preflight.md.'
    }
}

# Advisory only.
exit 0
