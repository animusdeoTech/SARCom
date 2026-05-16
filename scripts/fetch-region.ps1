# scripts/fetch-region.ps1 -- produces basemap.pmtiles (and optional
# hillshade.pmtiles) for a region under resources/regions/<region-name>/,
# per the convention in resources/regions/README.md.
#
# Usage:
#   scripts\fetch-region.ps1 <region-name>
#
# Per-product idempotency. The basemap bake is skipped when
# basemap.pmtiles already matches its expected sha256 (against
# region.toml [source].expected_sha256, or against basemap.provenance.json's
# last-recorded sha256). The hillshade bake is skipped when
# hillshade.pmtiles already matches hillshade.provenance.json's sha256.
#
# Basemap [source].kind values: url_fixture, protomaps_extract.
# Stubs: planetiler_bake, pmtiles_convert (error with clear message).
#
# Overlay [[overlays]] kinds processed:
#   kind = "osm"        no-op here (file is hand-drawn, committed)
#   kind = "hillshade"  + source = "dhmv_ii_dsm_1m":
#                       GDAL pipeline against DHMV-II 1m DSM tiles staged
#                       in $env:SARCOM_LIDAR_CACHE or
#                       %LOCALAPPDATA%\SARCom\lidar-cache\dhmv-ii\dsm\.
#                       Requires gdal_translate / gdalwarp / gdaldem /
#                       gdal2tiles.py on PATH; install OSGeo4W or
#                       `conda install -c conda-forge gdal`.

[CmdletBinding()]
param(
    [Parameter(Mandatory=$true, Position=0)]
    [string]$RegionName
)

$ErrorActionPreference = 'Stop'

$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$RegionDir = Join-Path $RepoRoot "resources/regions/$RegionName"
$TomlPath  = Join-Path $RegionDir 'region.toml'

if (-not (Test-Path $TomlPath)) {
    Write-Error "region.toml not found: $TomlPath"
    exit 2
}

$Content = Get-Content -Raw $TomlPath

# Minimal TOML scalar extractor for our flat [section] key = value shape.
# Handles both quoted strings ("foo") and bare scalars (5.420, 14, etc.).
# Not a full TOML parser; sufficient for the schema in
# resources/regions/README.md.
function Get-Scalar {
    param([string]$Section, [string]$Key)
    $sectionPattern = '(?ms)^\[' + [regex]::Escape($Section) + '\]\s*\r?\n(.*?)(\r?\n\[|\z)'
    if ($Content -match $sectionPattern) {
        $body = $Matches[1]
        $keyPattern = '(?m)^\s*' + [regex]::Escape($Key) + '\s*=\s*(?:"([^"]*)"|([^\s#"]+))'
        if ($body -match $keyPattern) {
            if ($Matches[1]) { return $Matches[1] }   # quoted form
            return $Matches[2]                         # bare scalar
        }
    }
    return $null
}

$Kind = Get-Scalar 'source' 'kind'
if (-not $Kind) {
    Write-Error "region.toml at $TomlPath must declare [source].kind"
    exit 3
}

$BasemapPath    = Join-Path $RegionDir 'basemap.pmtiles'
$ProvenancePath = Join-Path $RegionDir 'basemap.provenance.json'
$MismatchPath   = Join-Path $RegionDir 'provenance-mismatch.txt'
$TmpPath        = "$BasemapPath.tmp"

# Clear a stale mismatch marker before any new work.
if (Test-Path $MismatchPath) { Remove-Item $MismatchPath -Force }

switch ($Kind) {
    'url_fixture' {
        $Url                 = Get-Scalar 'source' 'url'
        $ExpectedSha256      = Get-Scalar 'source' 'expected_sha256'
        $License             = Get-Scalar 'source' 'license'
        $Attribution         = Get-Scalar 'source' 'attribution'
        $SourceExtractDate   = Get-Scalar 'source' 'source_extract_date'

        if (-not $Url) {
            Write-Error "[source].kind=url_fixture requires [source].url in $TomlPath"
            exit 4
        }

        # Idempotence --skip if existing basemap matches the active pin.
        if (Test-Path $BasemapPath) {
            $existingSha = (Get-FileHash -Algorithm SHA256 $BasemapPath).Hash.ToLower()
            if ($ExpectedSha256) {
                if ($existingSha -eq $ExpectedSha256.ToLower()) {
                    Write-Host "[skip] $RegionName/basemap.pmtiles matches expected_sha256."
                    $BasemapSkipped = $true
                    break
                }
            } elseif (Test-Path $ProvenancePath) {
                try {
                    $prev = Get-Content -Raw $ProvenancePath | ConvertFrom-Json
                    if ($prev.sha256 -and ($existingSha -eq $prev.sha256.ToLower())) {
                        Write-Host "[skip] $RegionName/basemap.pmtiles matches basemap.provenance.json sha256 (no expected_sha256 pin)."
                        $BasemapSkipped = $true
                        break
                    }
                } catch {
                    # Malformed provenance.json --fall through and re-fetch.
                }
            }
        }

        Write-Host "[fetch] $Url"
        $ProgressPreference = 'SilentlyContinue'
        Invoke-WebRequest -Uri $Url -OutFile $TmpPath -UseBasicParsing

        # PMTiles v3 magic bytes --see https://github.com/protomaps/PMTiles/blob/main/spec/v3/spec.md
        $magic = [System.IO.File]::ReadAllBytes($TmpPath)[0..6]
        $expected = @(0x50, 0x4D, 0x54, 0x69, 0x6C, 0x65, 0x73)   # "PMTiles"
        $mismatch = $false
        for ($i = 0; $i -lt 7; $i++) {
            if ($magic[$i] -ne $expected[$i]) { $mismatch = $true; break }
        }
        if ($mismatch) {
            Remove-Item $TmpPath -Force
            Write-Error "Downloaded file is not a PMTiles archive (magic bytes mismatch): $Url"
            exit 5
        }

        $sha256 = (Get-FileHash -Algorithm SHA256 $TmpPath).Hash.ToLower()
        $bytes  = (Get-Item $TmpPath).Length

        if ($ExpectedSha256 -and ($sha256 -ne $ExpectedSha256.ToLower())) {
            $msg = @"
expected_sha256 mismatch.
  region:   $RegionName
  url:      $Url
  expected: $ExpectedSha256
  actual:   $sha256
The downloaded file is at $TmpPath. Inspect manually --either pin the new
sha256 in region.toml or investigate the upstream change before keeping it.
"@
            [System.IO.File]::WriteAllText($MismatchPath, $msg, [System.Text.UTF8Encoding]::new($false))
            Write-Error "expected_sha256 mismatch --see $MismatchPath"
            exit 6
        }

        Move-Item -Force $TmpPath $BasemapPath

        $provenance = [ordered]@{
            region              = $RegionName
            fetched_at_utc      = (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ssZ')
            source_kind         = $Kind
            source_url          = $Url
            source_extract_date = $SourceExtractDate
            license             = $License
            attribution         = $Attribution
            sha256              = $sha256
            bytes               = $bytes
            tool                = 'scripts/fetch-region.ps1'
        }
        $json = $provenance | ConvertTo-Json -Depth 5
        [System.IO.File]::WriteAllText($ProvenancePath, $json, [System.Text.UTF8Encoding]::new($false))

        $sizeMb = [math]::Round($bytes / 1MB, 2)
        $shaShort = $sha256.Substring(0, 12)
        Write-Host ("[ok] {0} -> {1} ({2} MB, sha256 {3}...)" -f $RegionName, $BasemapPath, $sizeMb, $shaShort)
    }

    'protomaps_extract' {
        # Clip a small region out of a Protomaps planet-scale daily build via
        # HTTP range requests using the go-pmtiles CLI. No full-planet download.
        $UpstreamUrl       = Get-Scalar 'source' 'upstream_url'
        $ToolPin           = Get-Scalar 'source' 'tool'                       # e.g. "go-pmtiles@v1.30.2"
        $SourceExtractDate = Get-Scalar 'source' 'source_extract_date'
        $License           = Get-Scalar 'source' 'license'
        $Attribution       = Get-Scalar 'source' 'attribution'

        if (-not $UpstreamUrl) {
            Write-Error "[source].kind=protomaps_extract requires [source].upstream_url in $TomlPath"
            exit 4
        }

        # Bounds come from [bounds] in WGS84 — required by the spike's lens
        # Axis 2 amendment at spikes/pmtiles-walkers-spike.md:142.
        $MinLon = Get-Scalar 'bounds' 'min_lon'
        $MinLat = Get-Scalar 'bounds' 'min_lat'
        $MaxLon = Get-Scalar 'bounds' 'max_lon'
        $MaxLat = Get-Scalar 'bounds' 'max_lat'
        if (-not ($MinLon -and $MinLat -and $MaxLon -and $MaxLat)) {
            Write-Error "[bounds] block missing min_lon/min_lat/max_lon/max_lat in $TomlPath"
            exit 4
        }

        # go-pmtiles CLI is downloaded once into tools/bin and reused across
        # regions. Pin from region.toml; default to a working version.
        $PmtilesVersion = if ($ToolPin -match 'go-pmtiles@v?([0-9]+\.[0-9]+\.[0-9]+)') {
            $Matches[1]
        } else {
            '1.30.2'
        }
        $BinDir = Join-Path $RepoRoot 'tools/bin'
        $PmtilesExe = Join-Path $BinDir 'pmtiles.exe'
        $PmtilesVersionMarker = Join-Path $BinDir "pmtiles.exe.version"

        $needsDownload = $true
        if ((Test-Path $PmtilesExe) -and (Test-Path $PmtilesVersionMarker)) {
            $installed = (Get-Content -Raw $PmtilesVersionMarker).Trim()
            if ($installed -eq $PmtilesVersion) { $needsDownload = $false }
        }

        if ($needsDownload) {
            # First-install (or version-mismatch) confirmation. Pieter spends
            # disk on his terms, not on Claude's: don't silently drop a
            # vendored binary into the repo without an explicit y/N.
            Write-Host ""
            Write-Host "[install] $RegionName uses kind=protomaps_extract, which needs the go-pmtiles CLI."
            Write-Host "[install] Want to download go-pmtiles v$PmtilesVersion (~10 MB) from GitHub releases"
            Write-Host "[install] to $PmtilesExe?"
            $response = Read-Host "[y/N]"
            if ($response -notmatch '^[Yy]') {
                Write-Error "go-pmtiles not installed; cannot bake $RegionName. Re-run and confirm, or install pmtiles manually and place its binary at $PmtilesExe."
                exit 12
            }
            if (-not (Test-Path $BinDir)) { New-Item -ItemType Directory -Path $BinDir | Out-Null }
            $zipUrl = "https://github.com/protomaps/go-pmtiles/releases/download/v$PmtilesVersion/go-pmtiles_${PmtilesVersion}_Windows_x86_64.zip"
            $zipPath = Join-Path $BinDir "go-pmtiles_$PmtilesVersion.zip"
            Write-Host "[install] go-pmtiles v$PmtilesVersion -> $PmtilesExe"
            $ProgressPreference = 'SilentlyContinue'
            Invoke-WebRequest -Uri $zipUrl -OutFile $zipPath -UseBasicParsing
            Expand-Archive -Force -Path $zipPath -DestinationPath $BinDir
            # The zip extracts a 'pmtiles.exe' at the archive root.
            if (-not (Test-Path $PmtilesExe)) {
                Write-Error "Expected pmtiles.exe at $PmtilesExe after extracting $zipPath"
                exit 7
            }
            Remove-Item $zipPath -Force
            Set-Content -Encoding ascii $PmtilesVersionMarker $PmtilesVersion
        }

        # Bbox order for `pmtiles extract` is min_lon,min_lat,max_lon,max_lat (W,S,E,N).
        $bbox = "$MinLon,$MinLat,$MaxLon,$MaxLat"

        Write-Host "[extract] $UpstreamUrl"
        Write-Host "[extract] bbox=$bbox -> $TmpPath"
        $startedAt = Get-Date
        & $PmtilesExe extract $UpstreamUrl $TmpPath "--bbox=$bbox"
        if ($LASTEXITCODE -ne 0) {
            if (Test-Path $TmpPath) { Remove-Item $TmpPath -Force }
            Write-Error "pmtiles extract exited with code $LASTEXITCODE"
            exit 8
        }
        $elapsedSec = [int]((Get-Date) - $startedAt).TotalSeconds

        # Verify PMTiles magic on the extracted artifact.
        $magic = [System.IO.File]::ReadAllBytes($TmpPath)[0..6]
        $expected = @(0x50, 0x4D, 0x54, 0x69, 0x6C, 0x65, 0x73)
        $magicMismatch = $false
        for ($i = 0; $i -lt 7; $i++) {
            if ($magic[$i] -ne $expected[$i]) { $magicMismatch = $true; break }
        }
        if ($magicMismatch) {
            Remove-Item $TmpPath -Force
            Write-Error "Extracted file is not a PMTiles archive (magic bytes mismatch)."
            exit 5
        }

        $sha256 = (Get-FileHash -Algorithm SHA256 $TmpPath).Hash.ToLower()
        $bytes  = (Get-Item $TmpPath).Length

        Move-Item -Force $TmpPath $BasemapPath

        $provenance = [ordered]@{
            region              = $RegionName
            fetched_at_utc      = (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ssZ')
            source_kind         = $Kind
            source_upstream_url = $UpstreamUrl
            source_extract_date = $SourceExtractDate
            bbox_wgs84          = $bbox
            license             = $License
            attribution         = $Attribution
            sha256              = $sha256
            bytes               = $bytes
            bake_seconds        = $elapsedSec
            tool                = "go-pmtiles@v$PmtilesVersion"
            tool_invocation     = "pmtiles extract <upstream> <out> --bbox=$bbox"
            recipe              = 'scripts/fetch-region.ps1'
        }
        $json = $provenance | ConvertTo-Json -Depth 5
        [System.IO.File]::WriteAllText($ProvenancePath, $json, [System.Text.UTF8Encoding]::new($false))

        $sizeMb = [math]::Round($bytes / 1MB, 2)
        $shaShort = $sha256.Substring(0, 12)
        Write-Host ("[ok] {0} -> {1} ({2} MB, sha256 {3}..., {4}s)" -f $RegionName, $BasemapPath, $sizeMb, $shaShort, $elapsedSec)
    }

    'planetiler_bake' {
        Write-Error "kind=planetiler_bake not implemented in this revision (Phase 3 deliverable of spikes/pmtiles-walkers-spike.md)."
        exit 10
    }

    'pmtiles_convert' {
        Write-Error "kind=pmtiles_convert not implemented in this revision (Phase 3 deliverable of spikes/pmtiles-walkers-spike.md)."
        exit 10
    }

    default {
        Write-Error "Unknown [source].kind '$Kind' in $TomlPath"
        exit 11
    }
}

# ---------------------------------------------------------------------------
# Overlay processing: [[overlays]] entries from region.toml
# ---------------------------------------------------------------------------
# After the basemap is baked (or skipped as idempotent), iterate any
# [[overlays]] entries declared in region.toml and dispatch each:
#   kind = "osm"        no-op here (file is hand-drawn, committed to repo)
#   kind = "hillshade"  + source = "dhmv_ii_dsm_1m": run the GDAL pipeline
#                       below.
#
# Schema migrated in dev-log/2026-05-16-lidar-overlay-implementation.md;
# the producer half of the region-descriptor contract owned by
# spikes/field-deployment-package-shape-spike.md is extended here with
# (a) typed [[overlays]] array and (b) per-product provenance sidecars.

# Parse [[overlays]] array-of-tables from $Content.
# Each match returns a hashtable with the per-table scalar fields.
function Get-OverlayBlocks {
    $blocks = @()
    $blockPattern = '(?ms)^\[\[overlays\]\]\s*\r?\n(.*?)(?=\r?\n\[|\z)'
    $blockMatches = [regex]::Matches($Content, $blockPattern)
    foreach ($m in $blockMatches) {
        $body = $m.Groups[1].Value
        $entry = @{}
        $kvPattern = '(?m)^\s*([a-z_]+)\s*=\s*(?:"([^"]*)"|([^\s#"]+))'
        foreach ($lm in [regex]::Matches($body, $kvPattern)) {
            $key = $lm.Groups[1].Value
            $val = if ($lm.Groups[2].Success -and $lm.Groups[2].Value) {
                $lm.Groups[2].Value
            } else {
                $lm.Groups[3].Value
            }
            $entry[$key] = $val
        }
        $blocks += ,@($entry)
    }
    return ,$blocks
}

function Get-LidarCacheRoot {
    $root = $env:SARCOM_LIDAR_CACHE
    if ([string]::IsNullOrEmpty($root)) {
        $root = Join-Path $env:LOCALAPPDATA 'SARCom\lidar-cache'
    }
    if (-not (Test-Path $root)) {
        New-Item -ItemType Directory -Path $root -Force | Out-Null
    }
    return $root
}

function Get-DhmvDsmCacheDir {
    $dir = Join-Path (Get-LidarCacheRoot) 'dhmv-ii\dsm'
    if (-not (Test-Path $dir)) {
        New-Item -ItemType Directory -Path $dir -Force | Out-Null
    }
    return $dir
}

# Approximate WGS84 bboxes per DHMV-II kaartblad (NGI 1/40 sheet system).
# These values are LOW-CONFIDENCE; the script reports the sheets it thinks
# cover a region but the operator must verify against AGIV's portal map at
# https://www.geopunt.be/ before staging downloads. Expand this table as
# new regions need new sheets.
$DhmvSheetWgs84Bboxes = @{
    25 = @{ MinLon = 5.00; MinLat = 50.85; MaxLon = 5.55; MaxLat = 51.05; Label = 'k25 Hasselt area' }
    26 = @{ MinLon = 5.50; MinLat = 50.85; MaxLon = 6.05; MaxLat = 51.05; Label = 'k26 Genk / Maaseik area' }
    33 = @{ MinLon = 5.00; MinLat = 50.65; MaxLon = 5.55; MaxLat = 50.90; Label = 'k33 Sint-Truiden / Tongeren area' }
    34 = @{ MinLon = 5.50; MinLat = 50.65; MaxLon = 6.05; MaxLat = 50.90; Label = 'k34 Maaseik south area' }
}

function Test-BboxOverlap {
    param($A, $B)
    # Standard AABB overlap test (non-strict).
    return -not (
        $A.MaxLon -lt $B.MinLon -or
        $A.MinLon -gt $B.MaxLon -or
        $A.MaxLat -lt $B.MinLat -or
        $A.MinLat -gt $B.MaxLat
    )
}

function Find-DhmvSheetsForWgs84Bbox {
    param($MinLon, $MinLat, $MaxLon, $MaxLat)
    $regionBbox = @{ MinLon = $MinLon; MinLat = $MinLat; MaxLon = $MaxLon; MaxLat = $MaxLat }
    $sheets = @()
    foreach ($k in ($DhmvSheetWgs84Bboxes.Keys | Sort-Object)) {
        if (Test-BboxOverlap $regionBbox $DhmvSheetWgs84Bboxes[$k]) {
            $sheets += $k
        }
    }
    return ,$sheets
}

function Get-DhmvSheetPath {
    param([int]$SheetNumber)
    # AGIV naming convention: DHMVIIDSMRAS1m_kXX.tif. Sheet numbers are
    # two-digit zero-padded in the file name.
    $name = "DHMVIIDSMRAS1m_k{0:D2}.tif" -f $SheetNumber
    return Join-Path (Get-DhmvDsmCacheDir) $name
}

function Test-DhmvSheetsPresent {
    param($Sheets)
    # Returns a hashtable: sheet number -> file path if exists, else $null.
    $result = @{}
    foreach ($k in $Sheets) {
        $p = Get-DhmvSheetPath $k
        if (Test-Path $p) {
            $result[$k] = $p
        } else {
            $result[$k] = $null
        }
    }
    return $result
}

function Write-MissingSheetsInstructions {
    param($MissingSheets, $RegionName, $RegionBbox)
    Write-Host ""
    Write-Host "==============================================================="
    Write-Host " DHMV-II sheets not staged in the cache; operator action needed"
    Write-Host "==============================================================="
    Write-Host ""
    Write-Host "Region:    $RegionName"
    Write-Host "Bbox:      lon $($RegionBbox.MinLon)..$($RegionBbox.MaxLon), lat $($RegionBbox.MinLat)..$($RegionBbox.MaxLat) (WGS84)"
    Write-Host "Cache dir: $(Get-DhmvDsmCacheDir)"
    Write-Host ""
    Write-Host "Missing sheets (script's lookup; verify against the AGIV portal):"
    foreach ($k in $MissingSheets) {
        $label = $DhmvSheetWgs84Bboxes[$k].Label
        $name = "DHMVIIDSMRAS1m_k{0:D2}.tif" -f $k
        Write-Host "  - sheet $k  ($label)"
        Write-Host "    expected at: $(Get-DhmvSheetPath $k)"
    }
    Write-Host ""
    Write-Host "How to stage:"
    Write-Host "  1. Open https://www.geopunt.be/ (or the AGIV download portal)."
    Write-Host "  2. Log in via itsme/eID (DHMV-II requires authentication)."
    Write-Host "  3. Find 'Digitaal Hoogtemodel Vlaanderen II' -> 'DSM' -> 'raster' -> '1m'."
    Write-Host "  4. Select the kaartbladen listed above on the map."
    Write-Host "  5. Download the zip(s). Extract the DHMVIIDSMRAS1m_kXX.tif files"
    Write-Host "     into the cache directory shown above."
    Write-Host "  6. Re-run: scripts\fetch-region.ps1 $RegionName"
    Write-Host ""
    Write-Host "Sheet lookup is approximate. If the script's sheet list does not"
    Write-Host "match what the AGIV portal shows for your bbox, the table at"
    Write-Host "`$DhmvSheetWgs84Bboxes in scripts/fetch-region.ps1 needs expanding"
    Write-Host "or correcting."
    Write-Host ""
    Write-Host "==============================================================="
    Write-Host ""
}

function Test-GdalAvailable {
    $required = @('gdalwarp', 'gdaldem', 'gdal2tiles.py', 'gdal_translate')
    $missing = @()
    foreach ($t in $required) {
        # gdal2tiles.py needs Python; check that python finds it via PATH.
        # On OSGeo4W the .py is dispatched through a wrapper .bat.
        $cmd = Get-Command $t -ErrorAction SilentlyContinue
        if (-not $cmd) { $missing += $t }
    }
    if ($missing.Count -gt 0) {
        Write-Host ""
        Write-Host "==============================================================="
        Write-Host " GDAL tools not found on PATH"
        Write-Host "==============================================================="
        Write-Host ""
        Write-Host "Missing: $($missing -join ', ')"
        Write-Host ""
        Write-Host "Install one of:"
        Write-Host "  - OSGeo4W: https://trac.osgeo.org/osgeo4w/"
        Write-Host "    pick 'Express Install' -> 'GDAL'"
        Write-Host "  - Conda Forge: conda install -c conda-forge gdal"
        Write-Host ""
        Write-Host "After install: open a new shell so PATH refreshes,"
        Write-Host "then re-run scripts\fetch-region.ps1 $RegionName"
        Write-Host ""
        Write-Host "==============================================================="
        Write-Host ""
        return $false
    }
    return $true
}

function Get-CacheManifest {
    $manifestPath = Join-Path (Get-DhmvDsmCacheDir) 'cache-manifest.json'
    if (Test-Path $manifestPath) {
        try {
            return Get-Content -Raw $manifestPath | ConvertFrom-Json
        } catch {
            Write-Host "[warn] cache-manifest.json is malformed at $manifestPath; treating as empty"
        }
    }
    return [PSCustomObject]@{
        sheets        = @{}
        license_text  = $null
        license_path  = $null
    }
}

function Save-CacheManifest {
    param($Manifest)
    $manifestPath = Join-Path (Get-DhmvDsmCacheDir) 'cache-manifest.json'
    $json = $Manifest | ConvertTo-Json -Depth 6
    [System.IO.File]::WriteAllText($manifestPath, $json, [System.Text.UTF8Encoding]::new($false))
}

function Invoke-DhmvHillshadeBake {
    param(
        [string]$OverlayFile,
        [string]$OverlaySource
    )
    Write-Host ""
    Write-Host "[hillshade] starting DHMV-II hillshade bake for $RegionName"

    # 1. GDAL prereq check.
    if (-not (Test-GdalAvailable)) {
        exit 20
    }

    # 2. Read region bounds (script-scope $Content already loaded).
    $MinLon = [double](Get-Scalar 'bounds' 'min_lon')
    $MinLat = [double](Get-Scalar 'bounds' 'min_lat')
    $MaxLon = [double](Get-Scalar 'bounds' 'max_lon')
    $MaxLat = [double](Get-Scalar 'bounds' 'max_lat')

    # 3. Find covering sheets, check cache.
    $sheets = Find-DhmvSheetsForWgs84Bbox $MinLon $MinLat $MaxLon $MaxLat
    if ($sheets.Count -eq 0) {
        Write-Host "[hillshade] no DHMV sheet covers this region's bbox per the (approximate)"
        Write-Host "[hillshade] lookup at `$DhmvSheetWgs84Bboxes. Either the region is outside"
        Write-Host "[hillshade] Flanders (use a different source kind) or the lookup table needs"
        Write-Host "[hillshade] expanding."
        exit 21
    }

    $sheetPaths = Test-DhmvSheetsPresent $sheets
    $missing = @($sheetPaths.GetEnumerator() | Where-Object { $null -eq $_.Value } | ForEach-Object { $_.Key })
    if ($missing.Count -gt 0) {
        Write-MissingSheetsInstructions -MissingSheets $missing -RegionName $RegionName -RegionBbox @{ MinLon = $MinLon; MinLat = $MinLat; MaxLon = $MaxLon; MaxLat = $MaxLat }
        exit 22
    }

    $inputSheets = @($sheetPaths.GetEnumerator() | Sort-Object Key | ForEach-Object { $_.Value })
    Write-Host "[hillshade] using $($inputSheets.Count) sheet(s): $($sheets -join ', ')"

    # 4. Per-product idempotency check.
    $hillshadePath = Join-Path $RegionDir 'hillshade.pmtiles'
    $hillshadeProvenancePath = Join-Path $RegionDir 'hillshade.provenance.json'
    if (Test-Path $hillshadePath) {
        $existingSha = (Get-FileHash -Algorithm SHA256 $hillshadePath).Hash.ToLower()
        if (Test-Path $hillshadeProvenancePath) {
            try {
                $prev = Get-Content -Raw $hillshadeProvenancePath | ConvertFrom-Json
                if ($prev.sha256 -and ($existingSha -eq $prev.sha256.ToLower())) {
                    Write-Host "[skip] $RegionName/hillshade.pmtiles matches hillshade.provenance.json sha256"
                    return
                }
            } catch {}
        }
    }

    # 5. GDAL pipeline. Tmpdir under system temp.
    $tmpDir = Join-Path ([System.IO.Path]::GetTempPath()) ("sarcom-hillshade-$RegionName-" + [guid]::NewGuid().ToString('N').Substring(0, 8))
    New-Item -ItemType Directory -Path $tmpDir -Force | Out-Null
    $clipped = Join-Path $tmpDir 'dsm-clipped.tif'
    $hillshadeTif = Join-Path $tmpDir 'hillshade.tif'
    $tilesDir = Join-Path $tmpDir 'tiles'

    $startedAt = Get-Date

    try {
        # 5a. gdalwarp: clip to bbox + reproject EPSG:31370 -> EPSG:3857.
        # -te in EPSG:4326 (lat/lon) since region.toml bounds are WGS84.
        # NoData -9999 preserved through the warp.
        Write-Host "[hillshade] 1/4 gdalwarp clip + reproject ($($sheets -join ',') -> $clipped)"
        $warpArgs = @(
            '-t_srs', 'EPSG:3857',
            '-te', $MinLon, $MinLat, $MaxLon, $MaxLat,
            '-te_srs', 'EPSG:4326',
            '-srcnodata', '-9999',
            '-dstnodata', '-9999',
            '-r', 'bilinear',
            '-overwrite',
            '-q'
        ) + $inputSheets + @($clipped)
        & gdalwarp @warpArgs
        if ($LASTEXITCODE -ne 0) { throw "gdalwarp exited with code $LASTEXITCODE" }

        # 5b. gdaldem hillshade.
        Write-Host "[hillshade] 2/4 gdaldem hillshade (az=315 alt=45)"
        & gdaldem hillshade -compute_edges -az 315 -alt 45 -z 1.0 -q $clipped $hillshadeTif
        if ($LASTEXITCODE -ne 0) { throw "gdaldem exited with code $LASTEXITCODE" }

        # 5c. gdal2tiles.py: XYZ pyramid (Slippy) z0-15.
        Write-Host "[hillshade] 3/4 gdal2tiles.py z0-15 (this is the slow step; ~30-90s)"
        # Invoke via `python -m osgeo_utils.gdal2tiles` rather than `gdal2tiles.py` directly.
        # On Windows, `.py` files are dispatched through the file association which may not
        # actually execute via Python (silent no-op, $LASTEXITCODE stays 0, downstream step
        # 4/4 then fails on missing tiles directory). The module-path invocation is portable
        # across the conda-forge GDAL install (GDAL 3.x ships gdal2tiles under osgeo_utils).
        # --webviewer=none skips MapML / Leaflet / OpenLayers HTML viewer generation, which
        # (a) we don't need (pmtiles convert only reads the XYZ tile tree), and (b) requires
        # GDAL_DATA to be set in the env to find the MapML templates — conda's activation
        # hook would set it but we add the conda dirs to PATH manually without running
        # `conda activate`, so GDAL_DATA stays unset and the template lookup crashes
        # (TypeError: expected str, bytes or os.PathLike object, not NoneType).
        # Quiet flag dropped so genuine gdal2tiles errors surface in this terminal.
        & python -m osgeo_utils.gdal2tiles --xyz --zoom=0-15 --resampling=bilinear --webviewer=none $hillshadeTif $tilesDir
        if ($LASTEXITCODE -ne 0) { throw "gdal2tiles (osgeo_utils) exited with code $LASTEXITCODE" }

        # 5d. XYZ tile directory -> MBTILES (via mb-util) -> PMTiles (via go-pmtiles).
        # go-pmtiles `convert` (as of v1.30.x) only accepts MBTILES file input, not XYZ
        # directories — it errors with "exists but is a directory" otherwise. mb-util
        # is the standard Python tool that packs an XYZ tile tree into an MBTILES SQLite
        # file in one shot. Install once into the same conda env: `pip install mbutil`.
        #
        # On Windows + conda, mb-util ships as a bare Python script in
        # %CONDA%\Scripts\mb-util (no .exe wrapper, no .py extension). PowerShell's
        # `& mb-util` cannot dispatch it directly: it appears to run but produces no
        # output and leaves $LASTEXITCODE at whatever the previous call set (silent
        # no-op, same failure shape as the gdal2tiles .py-association quirk in step 5c).
        # `python -m mbutil` also fails because the mbutil package lacks __main__.py
        # (errors: "'mbutil' is a package and cannot be directly executed").
        # Resolution: locate the script via Get-Command and invoke through python.
        $mbtilesPath = Join-Path $tmpDir 'hillshade.mbtiles'
        $pmtilesExe = Join-Path $RepoRoot 'tools\bin\pmtiles.exe'
        if (-not (Test-Path $pmtilesExe)) {
            throw "pmtiles.exe not found at $pmtilesExe; run a basemap bake first to auto-install go-pmtiles, or install it manually"
        }
        $mbUtilCmd = Get-Command mb-util -ErrorAction SilentlyContinue
        if (-not $mbUtilCmd) {
            throw "mb-util script not on PATH; install once with: pip install mbutil (into the same env that hosts gdal2tiles)"
        }
        Write-Host "[hillshade] 4/4 mb-util XYZ -> MBTILES, gzip-wrap, pmtiles convert -> PMTiles, fix-up header"
        & python $mbUtilCmd.Source --scheme=xyz $tilesDir $mbtilesPath
        if ($LASTEXITCODE -ne 0) {
            throw "mb-util (XYZ -> MBTILES) exited with code $LASTEXITCODE"
        }
        if (-not (Test-Path $mbtilesPath)) {
            throw "mb-util reported success but produced no $mbtilesPath -- check the XYZ tile tree at $tilesDir"
        }

        # Walkers PNG-via-PmTiles workaround: walkers-0.53.0/src/pmtiles.rs:127-155
        # calls pmtiles' raw `get_tile()` then runs an unconditional GzDecoder
        # over the bytes. Raster PNG tiles fail that step and silently drop.
        # Pre-gzip every MBTILES tile blob so walkers' decompress yields plain
        # PNG downstream, which walkers' Tile::new auto-detects as raster.
        # See dev-log/2026-05-16-lidar-overlay-implementation.md
        # `## Hillshade load-vs-render gap (2026-05-16)` for the full diagnosis.
        $gzipWrapScript = Join-Path $RepoRoot 'scripts\_gzip_mbtiles_tiles.py'
        Write-Host "[hillshade]    walkers workaround: gzip-wrap MBTILES tile blobs"
        & python $gzipWrapScript $mbtilesPath
        if ($LASTEXITCODE -ne 0) { throw "_gzip_mbtiles_tiles.py exited with code $LASTEXITCODE" }

        & $pmtilesExe convert $mbtilesPath $hillshadePath
        if ($LASTEXITCODE -ne 0) { throw "pmtiles convert (MBTILES -> PMTiles) exited with code $LASTEXITCODE" }

        # `pmtiles convert` leaves tile_compression="unknown" and tile_type=""
        # for non-MVT MBTILES inputs. Flip both so `pmtiles show` reports the
        # truth: gzip-wrapped PNG raster tiles. Round-trip the existing
        # header-json so we only edit the two fields and preserve bounds,
        # minzoom/maxzoom, center.
        Write-Host "[hillshade]    fix-up archive header: tile_compression=gzip, tile_type=png"
        $headerOut = & $pmtilesExe show --header-json $hillshadePath
        if ($LASTEXITCODE -ne 0) { throw "pmtiles show --header-json exited with code $LASTEXITCODE" }
        $header = $headerOut | ConvertFrom-Json
        $header.tile_compression = 'gzip'
        $header.tile_type        = 'png'
        $headerJsonPath = Join-Path $tmpDir 'header.json'
        $headerJson = $header | ConvertTo-Json -Depth 5 -Compress
        [System.IO.File]::WriteAllText($headerJsonPath, $headerJson, [System.Text.UTF8Encoding]::new($false))
        & $pmtilesExe edit "--header-json=$headerJsonPath" $hillshadePath
        if ($LASTEXITCODE -ne 0) { throw "pmtiles edit (set tile_compression/tile_type) exited with code $LASTEXITCODE" }
    } finally {
        # Cleanup tmpdir even on failure.
        if (Test-Path $tmpDir) {
            try { Remove-Item -Recurse -Force $tmpDir } catch {}
        }
    }

    $elapsedSec = [int]((Get-Date) - $startedAt).TotalSeconds
    $sha256 = (Get-FileHash -Algorithm SHA256 $hillshadePath).Hash.ToLower()
    $bytes  = (Get-Item $hillshadePath).Length

    # 6. License text: read from cache-manifest.json if the operator
    # captured it at sheet-staging time; otherwise note the gap. Operator
    # is expected to drop the license PDF / text-extract into the cache
    # manifest after first download (see QUICKSTART).
    $manifest = Get-CacheManifest
    $licenseText = if ($manifest.license_text) { $manifest.license_text } else {
        "Pending operator capture from Gebruik_DHMVIIDSMRAS1m.pdf at $(Get-DhmvDsmCacheDir). Update cache-manifest.json after first download; re-run bake to refresh."
    }

    # 7. Provenance sidecar.
    $hillshadeProvenance = [ordered]@{
        region              = $RegionName
        product             = 'hillshade'
        fetched_at_utc      = (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ssZ')
        source_kind         = $OverlaySource
        source_kaartbladen  = @($sheets)
        source_cache_dir    = (Get-DhmvDsmCacheDir)
        upstream_portal     = 'https://www.geopunt.be/'
        bbox_wgs84          = "$MinLon,$MinLat,$MaxLon,$MaxLat"
        crs_src             = 'EPSG:31370'
        crs_dst             = 'EPSG:3857'
        sha256              = $sha256
        bytes               = $bytes
        bake_seconds        = $elapsedSec
        tool_invocation     = "gdalwarp -> gdaldem hillshade (az=315 alt=45) -> gdal2tiles.py z0-15 --xyz -> pmtiles convert"
        recipe              = 'scripts/fetch-region.ps1'
        license_text        = $licenseText
    }
    $json = $hillshadeProvenance | ConvertTo-Json -Depth 6
    [System.IO.File]::WriteAllText($hillshadeProvenancePath, $json, [System.Text.UTF8Encoding]::new($false))

    $sizeMb = [math]::Round($bytes / 1MB, 2)
    $shaShort = $sha256.Substring(0, 12)
    Write-Host ("[ok] hillshade -> {0} ({1} MB, sha256 {2}..., {3}s)" -f $hillshadePath, $sizeMb, $shaShort, $elapsedSec)
}

# Dispatch overlay-baking work.
$overlays = Get-OverlayBlocks
foreach ($overlay in $overlays) {
    $kind = $overlay.kind
    switch ($kind) {
        'osm' {
            # No-op. OSM files are hand-drawn and committed to the repo.
            # The kiosk-lab discovers them through region.osm_overlay_path().
        }
        'hillshade' {
            $source = $overlay.source
            switch ($source) {
                'dhmv_ii_dsm_1m' {
                    Invoke-DhmvHillshadeBake -OverlayFile $overlay.file -OverlaySource $source
                }
                default {
                    Write-Host "[warn] [[overlays]] kind=hillshade source='$source' not implemented; skipping"
                }
            }
        }
        default {
            Write-Host "[warn] unknown [[overlays]] kind='$kind'; skipping"
        }
    }
}
