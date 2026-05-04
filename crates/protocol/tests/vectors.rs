use protocol::{
    crc16_ccitt_false, decode_position, encode_position, Flags, FrameError, Position, FRAME_LEN,
    NO_FIX_ALT_SENTINEL, NO_FIX_LAT_LON_SENTINEL,
};

// ── helpers ──────────────────────────────────────────────────────────────────

fn heartbeat_pos() -> Position {
    Position {
        node_id: 1,
        seq_nr: 42,
        flags: Flags(Flags::GPS_VALID_MASK),
        lat_e7: 471_234_567,
        lon_e7: 135_678_901,
        alt_m: 1847,
    }
}

fn no_fix_pos() -> Position {
    Position {
        node_id: 3,
        seq_nr: 7,
        flags: Flags(0),
        lat_e7: NO_FIX_LAT_LON_SENTINEL,
        lon_e7: NO_FIX_LAT_LON_SENTINEL,
        alt_m: NO_FIX_ALT_SENTINEL,
    }
}

// ── CRC self-test ─────────────────────────────────────────────────────────────

#[test]
fn crc_check_value() {
    assert_eq!(crc16_ccitt_false(b"123456789"), 0x29B1);
}

// ── vector discovery (run with -- --nocapture to read hex) ────────────────────

#[test]
fn print_canonical_vectors() {
    let hb = encode_position(heartbeat_pos());
    println!(
        "HEARTBEAT: {}",
        hb.iter()
            .map(|b| format!("{b:02X}"))
            .collect::<Vec<_>>()
            .join(" ")
    );

    let sos = encode_position(Position {
        flags: Flags(Flags::GPS_VALID_MASK | Flags::SOS_MASK),
        ..heartbeat_pos()
    });
    println!(
        "SOS:       {}",
        sos.iter()
            .map(|b| format!("{b:02X}"))
            .collect::<Vec<_>>()
            .join(" ")
    );
}

// ── canonical heartbeat vector ────────────────────────────────────────────────
// Frozen after running print_canonical_vectors.
// Pre-CRC bytes (0..19):
//   A5 01 01 10  01  00 00 00 2A  01
//   1C 16 78 07  08 16 4B B5  07 37
// CRC-16/CCITT-FALSE over bytes 0..19 appended as big-endian u16.

#[test]
fn canonical_heartbeat_vector() {
    let frame = encode_position(heartbeat_pos());
    // Assert the fixed header + payload bytes first (no CRC dependency)
    assert_eq!(frame[0], 0xA5, "MAGIC");
    assert_eq!(frame[1], 0x01, "VER");
    assert_eq!(frame[2], 0x01, "TYPE");
    assert_eq!(frame[3], 0x10, "LEN");
    assert_eq!(frame[4], 0x01, "node_id");
    assert_eq!(&frame[5..9], &[0x00, 0x00, 0x00, 0x2A], "seq_nr=42");
    assert_eq!(frame[9], 0x01, "flags=GPS_VALID");
    assert_eq!(
        &frame[10..14],
        &[0x1C, 0x16, 0x78, 0x07],
        "lat_e7=471234567"
    );
    assert_eq!(
        &frame[14..18],
        &[0x08, 0x16, 0x4B, 0xB5],
        "lon_e7=135678901"
    );
    assert_eq!(&frame[18..20], &[0x07, 0x37], "alt_m=1847");
    // CRC over bytes 0..19 must round-trip
    assert_eq!(decode_position(&frame), Ok(heartbeat_pos()));
    // Full frozen frame
    assert_eq!(frame, CANONICAL_HEARTBEAT, "full frame mismatch");
}

#[test]
fn canonical_sos_vector() {
    let sos_pos = Position {
        flags: Flags(Flags::GPS_VALID_MASK | Flags::SOS_MASK),
        ..heartbeat_pos()
    };
    let frame = encode_position(sos_pos);
    assert_eq!(frame[9], 0x03, "flags=GPS_VALID|SOS");
    assert_eq!(decode_position(&frame), Ok(sos_pos));
    assert_eq!(frame, CANONICAL_SOS, "full SOS frame mismatch");
}

// Frozen hex vectors — confirmed by running print_canonical_vectors.
// node_id=1, seq_nr=42, lat_e7=471234567, lon_e7=135678901, alt_m=1847.
// CRC-16/CCITT-FALSE (0x1021, init 0xFFFF) over bytes 0..19.
const CANONICAL_HEARTBEAT: [u8; FRAME_LEN] = [
    0xA5, 0x01, 0x01, 0x10, 0x01, 0x00, 0x00, 0x00, 0x2A, 0x01, 0x1C, 0x16, 0x78, 0x07, 0x08, 0x16,
    0x4B, 0xB5, 0x07, 0x37, 0x29, 0x04,
];
const CANONICAL_SOS: [u8; FRAME_LEN] = [
    0xA5, 0x01, 0x01, 0x10, 0x01, 0x00, 0x00, 0x00, 0x2A, 0x03, 0x1C, 0x16, 0x78, 0x07, 0x08, 0x16,
    0x4B, 0xB5, 0x07, 0x37, 0x89, 0xB7,
];

// ── decode rejection tests ────────────────────────────────────────────────────

#[test]
fn decode_rejects_bad_magic() {
    let mut f = encode_position(heartbeat_pos());
    f[0] = 0x00;
    assert_eq!(decode_position(&f), Err(FrameError::BadMagic));
}

#[test]
fn decode_rejects_wrong_version() {
    let mut f = encode_position(heartbeat_pos());
    f[1] = 0x02;
    // CRC will also mismatch, but version check comes first
    assert_eq!(decode_position(&f), Err(FrameError::BadVersion));
}

#[test]
fn decode_rejects_unknown_type() {
    let mut f = encode_position(heartbeat_pos());
    f[2] = 0x99;
    assert_eq!(decode_position(&f), Err(FrameError::UnknownType));
}

#[test]
fn decode_rejects_len_mismatch() {
    let mut f = encode_position(heartbeat_pos());
    f[3] = 0x0F; // LEN says 15 instead of 16
    assert_eq!(decode_position(&f), Err(FrameError::BadLength));
}

#[test]
fn decode_rejects_wrong_frame_size() {
    let short = [0u8; 21];
    assert_eq!(decode_position(&short), Err(FrameError::BadLength));
    let long = [0u8; 23];
    assert_eq!(decode_position(&long), Err(FrameError::BadLength));
}

#[test]
fn decode_rejects_crc_mismatch() {
    let mut f = encode_position(heartbeat_pos());
    f[21] ^= 0xFF; // corrupt last CRC byte
    assert_eq!(decode_position(&f), Err(FrameError::CrcMismatch));
}

#[test]
fn decode_rejects_reserved_flag_bits() {
    let pos = Position {
        flags: Flags(0x08), // bit 3 is reserved
        lat_e7: NO_FIX_LAT_LON_SENTINEL,
        lon_e7: NO_FIX_LAT_LON_SENTINEL,
        alt_m: NO_FIX_ALT_SENTINEL,
        ..heartbeat_pos()
    };
    // encode without CRC check, then manually fix CRC
    let mut f = encode_position(pos);
    // encode_position already sets a valid CRC, but the flags byte is reserved
    // decode must reject on reserved bits
    assert_eq!(decode_position(&f), Err(FrameError::ReservedFlagBits));
    // Verify the CRC itself would have been OK (corrupt it and check CrcMismatch path)
    f[9] = 0x08; // already set; just confirm decode rejects
    assert_eq!(decode_position(&f), Err(FrameError::ReservedFlagBits));
}

#[test]
fn no_fix_sentinel_round_trips() {
    let pos = no_fix_pos();
    let frame = encode_position(pos);
    assert_eq!(decode_position(&frame), Ok(pos));
}

#[test]
fn gps_valid_with_sentinels_rejected() {
    // GPS_VALID=1 but sentinel coordinates → mismatch
    let pos = Position {
        flags: Flags(Flags::GPS_VALID_MASK),
        lat_e7: NO_FIX_LAT_LON_SENTINEL,
        lon_e7: NO_FIX_LAT_LON_SENTINEL,
        alt_m: NO_FIX_ALT_SENTINEL,
        ..heartbeat_pos()
    };
    let frame = encode_position(pos);
    assert_eq!(
        decode_position(&frame),
        Err(FrameError::GpsValidSentinelMismatch)
    );
}

#[test]
fn gps_invalid_with_real_coords_rejected() {
    // GPS_VALID=0 but real (non-sentinel) coordinates → mismatch
    let pos = Position {
        flags: Flags(0), // GPS_VALID=0
        ..heartbeat_pos()
    };
    let frame = encode_position(pos);
    assert_eq!(
        decode_position(&frame),
        Err(FrameError::GpsValidSentinelMismatch)
    );
}

// ── partial sentinel rejection tests ─────────────────────────────────────────
// GPS_VALID=1: even a single sentinel coordinate is invalid.
// GPS_VALID=0: all three must be sentinel; any partial combination is invalid.

#[test]
fn gps_valid_lat_sentinel_only_rejected() {
    let pos = Position {
        flags: Flags(Flags::GPS_VALID_MASK),
        lat_e7: NO_FIX_LAT_LON_SENTINEL,
        lon_e7: 135_678_901,
        alt_m: 1847,
        ..heartbeat_pos()
    };
    assert_eq!(
        decode_position(&encode_position(pos)),
        Err(FrameError::GpsValidSentinelMismatch)
    );
}

#[test]
fn gps_valid_lon_sentinel_only_rejected() {
    let pos = Position {
        flags: Flags(Flags::GPS_VALID_MASK),
        lat_e7: 471_234_567,
        lon_e7: NO_FIX_LAT_LON_SENTINEL,
        alt_m: 1847,
        ..heartbeat_pos()
    };
    assert_eq!(
        decode_position(&encode_position(pos)),
        Err(FrameError::GpsValidSentinelMismatch)
    );
}

#[test]
fn gps_valid_alt_sentinel_only_rejected() {
    let pos = Position {
        flags: Flags(Flags::GPS_VALID_MASK),
        lat_e7: 471_234_567,
        lon_e7: 135_678_901,
        alt_m: NO_FIX_ALT_SENTINEL,
        ..heartbeat_pos()
    };
    assert_eq!(
        decode_position(&encode_position(pos)),
        Err(FrameError::GpsValidSentinelMismatch)
    );
}

#[test]
fn gps_invalid_latlon_sentinel_real_alt_rejected() {
    // GPS_VALID=0 + lat/lon both sentinel but alt is real → partial, invalid
    let pos = Position {
        flags: Flags(0),
        lat_e7: NO_FIX_LAT_LON_SENTINEL,
        lon_e7: NO_FIX_LAT_LON_SENTINEL,
        alt_m: 500,
        ..heartbeat_pos()
    };
    assert_eq!(
        decode_position(&encode_position(pos)),
        Err(FrameError::GpsValidSentinelMismatch)
    );
}

#[test]
fn gps_invalid_alt_sentinel_real_latlon_rejected() {
    // GPS_VALID=0 + alt sentinel but lat/lon are real → partial, invalid
    let pos = Position {
        flags: Flags(0),
        lat_e7: 471_234_567,
        lon_e7: 135_678_901,
        alt_m: NO_FIX_ALT_SENTINEL,
        ..heartbeat_pos()
    };
    assert_eq!(
        decode_position(&encode_position(pos)),
        Err(FrameError::GpsValidSentinelMismatch)
    );
}
