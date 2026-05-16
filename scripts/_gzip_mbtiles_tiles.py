"""Gzip-wrap every tile blob inside an MBTILES SQLite file, in place.

Workaround for walkers-0.53.0/src/pmtiles.rs:127-155, which calls the pmtiles
crate's raw `get_tile()` (no header-aware decompression) and then runs an
unconditional `flate2::read::GzDecoder` over the bytes. Raster PNG tiles stored
without a gzip wrap fail that step (gzip magic `1f 8b` vs PNG magic `89 50`),
walkers returns `PmTilesError::Decompression`, the tile is silently dropped,
and no hillshade pixels paint -- even though the layer is loaded and added via
`Map::with_layer`.

Pre-wrapping each tile blob inside MBTILES (before `pmtiles convert` lifts the
archive into PMTiles) makes walkers' decompress step yield plain PNG bytes
downstream, which walkers' `Tile::new` (walkers/src/tiles.rs:115-130) auto-
detects as raster and routes through the `Tile::Raster(TextureHandle)` path.

Idempotent: tiles already starting with the gzip magic are skipped, so re-
running the bake against a half-wrapped MBTILES is safe.
"""
import gzip
import io
import sqlite3
import sys

GZIP_MAGIC_0 = 0x1F
GZIP_MAGIC_1 = 0x8B


def main(path: str) -> int:
    conn = sqlite3.connect(path)
    cur = conn.cursor()

    # mb-util's default output is the "flat" MBTILES schema: a real `tiles`
    # table with (zoom_level, tile_column, tile_row, tile_data). Dedup-mode
    # MBTILES instead exposes `tiles` as a view over `images(tile_id, tile_data)`
    # joined through `map`; UPDATE-through-view is rejected by SQLite, so the
    # underlying `images` table must be updated instead.
    cur.execute(
        "SELECT name, type FROM sqlite_master WHERE name IN ('tiles', 'images')"
    )
    objs = {row[0]: row[1] for row in cur.fetchall()}
    if 'images' in objs:
        table = 'images'
        select_sql = "SELECT rowid, tile_data FROM images"
        update_sql = "UPDATE images SET tile_data = ? WHERE rowid = ?"
    elif 'tiles' in objs and objs['tiles'] == 'table':
        table = 'tiles'
        select_sql = "SELECT rowid, tile_data FROM tiles"
        update_sql = "UPDATE tiles SET tile_data = ? WHERE rowid = ?"
    else:
        print(
            "[gzip-wrap] error: neither `tiles` table nor `images` table found"
            f" in {path}",
            file=sys.stderr,
        )
        return 1

    cur.execute(select_sql)
    rows = cur.fetchall()
    print(f"[gzip-wrap] scanning {len(rows)} blob row(s) in `{table}`")

    updated = 0
    skipped = 0
    for rowid, data in rows:
        if data is None or len(data) < 2:
            skipped += 1
            continue
        if data[0] == GZIP_MAGIC_0 and data[1] == GZIP_MAGIC_1:
            skipped += 1
            continue
        buf = io.BytesIO()
        # mtime=0 keeps the gzip header deterministic so the resulting
        # PMTiles archive's sha256 is stable across re-bakes of the same
        # input tiles.
        with gzip.GzipFile(fileobj=buf, mode='wb', mtime=0) as gz:
            gz.write(data)
        cur.execute(update_sql, (sqlite3.Binary(buf.getvalue()), rowid))
        updated += 1

    conn.commit()
    conn.close()
    print(
        f"[gzip-wrap] gzipped {updated} blob(s); skipped {skipped}"
        " (empty or already-gzip)"
    )
    return 0


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(
            "usage: _gzip_mbtiles_tiles.py <path/to/file.mbtiles>",
            file=sys.stderr,
        )
        sys.exit(2)
    sys.exit(main(sys.argv[1]))
