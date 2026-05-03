#!/usr/bin/env python3
"""
Download OSM raster tiles for Terril Waterschei, Genk, Belgium.
Writes resources/tiles/terril_waterschei.mbtiles (plain SQLite, no extra packages).

After this script finishes, run:
    pmtiles convert resources/tiles/terril_waterschei.mbtiles \
                    resources/tiles/terril_waterschei.pmtiles

Get pmtiles.exe (Windows) from:
    https://github.com/protomaps/go-pmtiles/releases  →  pmtiles_windows_amd64.zip
"""

import math
import sqlite3
import time
import urllib.request
from pathlib import Path

# ── Bounding box ──────────────────────────────────────────────────────────────
# OSM extract is ~50.9998–51.0161 N, 5.5352–5.5571 E.
# Padded here for context (shows surrounding terrain, roads, the mine site).
MINLAT, MINLON = 50.988, 5.520
MAXLAT, MAXLON = 51.028, 5.580

MIN_ZOOM = 13   # regional overview
MAX_ZOOM = 18   # footpath-level detail (~0.6 m/px at lat 51°)

# ── Tile source ───────────────────────────────────────────────────────────────
# tile.openstreetmap.org is the standard OSM raster tile server.
# Policy: valid User-Agent required, no bulk automation, rate-limit respected.
# ~2700 tiles total for this bbox — well within acceptable one-off use.
TILE_URL   = "https://tile.openstreetmap.org/{z}/{x}/{y}.png"
USER_AGENT = "SARCOM-kiosk-lab/0.1 offline-tile-preloader (single-use)"
DELAY_S    = 0.20   # seconds between fetches — be polite

# ── Paths ─────────────────────────────────────────────────────────────────────
SCRIPT_DIR = Path(__file__).resolve().parent
OUT_DIR    = SCRIPT_DIR.parent / "resources" / "tiles"
OUT_MBTILES = OUT_DIR / "terril_waterschei.mbtiles"

# ── Tile math ─────────────────────────────────────────────────────────────────

def tile_xy(lat: float, lon: float, z: int) -> tuple[int, int]:
    """Slippy-map XYZ tile coordinates (standard OSM convention)."""
    n = 1 << z
    x = int((lon + 180.0) / 360.0 * n)
    lat_r = math.radians(lat)
    y = int((1.0 - math.log(math.tan(lat_r) + 1.0 / math.cos(lat_r)) / math.pi) / 2.0 * n)
    return x, y


def tms_y(xyz_y: int, z: int) -> int:
    """XYZ y → TMS y (MBTiles stores TMS convention: origin at bottom-left)."""
    return (1 << z) - 1 - xyz_y


def bbox_tiles(z: int) -> tuple[int, int, int, int]:
    """Return (x0, y0, x1, y1) in XYZ coords covering the bbox at zoom z."""
    x0, y0 = tile_xy(MAXLAT, MINLON, z)   # NW → smallest x, smallest y
    x1, y1 = tile_xy(MINLAT, MAXLON, z)   # SE → largest  x, largest  y
    return x0, y0, x1, y1


def total_tile_count() -> int:
    n = 0
    for z in range(MIN_ZOOM, MAX_ZOOM + 1):
        x0, y0, x1, y1 = bbox_tiles(z)
        n += (x1 - x0 + 1) * (y1 - y0 + 1)
    return n

# ── MBTiles helpers ───────────────────────────────────────────────────────────

def open_mbtiles(path: Path) -> sqlite3.Connection:
    conn = sqlite3.connect(path)
    conn.executescript("""
        CREATE TABLE IF NOT EXISTS metadata (name TEXT PRIMARY KEY, value TEXT);
        CREATE TABLE IF NOT EXISTS tiles (
            zoom_level  INTEGER,
            tile_column INTEGER,
            tile_row    INTEGER,
            tile_data   BLOB
        );
        CREATE UNIQUE INDEX IF NOT EXISTS tile_idx
            ON tiles(zoom_level, tile_column, tile_row);
    """)
    rows = [
        ("name",        "terril_waterschei"),
        ("type",        "baselayer"),
        ("format",      "png"),
        ("bounds",      f"{MINLON},{MINLAT},{MAXLON},{MAXLAT}"),
        ("center",      f"{(MINLON + MAXLON) / 2:.6f},{(MINLAT + MAXLAT) / 2:.6f},{MAX_ZOOM}"),
        ("minzoom",     str(MIN_ZOOM)),
        ("maxzoom",     str(MAX_ZOOM)),
        ("description", "OpenStreetMap raster tiles — Terril Waterschei, Genk, Belgium"),
    ]
    conn.executemany("INSERT OR REPLACE INTO metadata VALUES (?,?)", rows)
    conn.commit()
    return conn


def already_have(cur: sqlite3.Cursor, z: int, x: int, tms_row: int) -> bool:
    cur.execute(
        "SELECT 1 FROM tiles WHERE zoom_level=? AND tile_column=? AND tile_row=?",
        (z, x, tms_row),
    )
    return cur.fetchone() is not None


def fetch_tile(z: int, x: int, y: int) -> bytes:
    url = TILE_URL.format(z=z, x=x, y=y)
    req = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    with urllib.request.urlopen(req, timeout=20) as r:
        return r.read()

# ── Main ──────────────────────────────────────────────────────────────────────

def main() -> None:
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    conn = open_mbtiles(OUT_MBTILES)
    cur  = conn.cursor()

    total    = total_tile_count()
    done     = 0
    skipped  = 0
    errors   = 0

    print(f"Bbox:   {MINLAT}–{MAXLAT} N, {MINLON}–{MAXLON} E")
    print(f"Zooms:  {MIN_ZOOM}–{MAX_ZOOM}   |   tiles: ~{total}")
    print(f"Output: {OUT_MBTILES}\n")

    for z in range(MIN_ZOOM, MAX_ZOOM + 1):
        x0, y0, x1, y1 = bbox_tiles(z)
        nrow = (x1 - x0 + 1) * (y1 - y0 + 1)
        print(f"  z{z:2d}  {x1 - x0 + 1} × {y1 - y0 + 1} = {nrow} tiles")

        for x in range(x0, x1 + 1):
            for y in range(y0, y1 + 1):
                ty = tms_y(y, z)

                if already_have(cur, z, x, ty):
                    skipped += 1
                    done    += 1
                    continue

                try:
                    data = fetch_tile(z, x, y)
                    cur.execute(
                        "INSERT OR REPLACE INTO tiles VALUES (?,?,?,?)",
                        (z, x, ty, data),
                    )
                    conn.commit()
                    done += 1
                    pct  = done * 100 // total
                    print(f"\r    [{pct:3d}%] {done}/{total}  z{z} {x}/{y}   ", end="", flush=True)
                    time.sleep(DELAY_S)

                except Exception as exc:
                    errors += 1
                    print(f"\n    WARN z{z}/{x}/{y}: {exc}")

        print()  # newline after each zoom level

    conn.close()

    downloaded = done - skipped
    print(f"\nDone.  {downloaded} downloaded  |  {skipped} already cached  |  {errors} errors")
    print(f"File:  {OUT_MBTILES}  ({OUT_MBTILES.stat().st_size // 1024} KB)\n")
    print("Next step — convert to PMTiles:")
    print(f'    pmtiles convert "{OUT_MBTILES}" "{OUT_DIR / "terril_waterschei.pmtiles"}"')


if __name__ == "__main__":
    main()
