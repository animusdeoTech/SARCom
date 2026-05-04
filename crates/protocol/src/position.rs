use crate::crc::crc16_ccitt_false;
use crate::flags::Flags;
use crate::frame::{
    FrameError, PacketId, FRAME_LEN, MAGIC, NO_FIX_ALT_SENTINEL, NO_FIX_LAT_LON_SENTINEL,
    PAYLOAD_LEN, TYPE_POSITION, VERSION,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub node_id: u8,
    pub seq_nr: u32,
    pub flags: Flags,
    pub lat_e7: i32,
    pub lon_e7: i32,
    pub alt_m: i16,
}

pub fn encode_position(p: Position) -> [u8; FRAME_LEN] {
    let mut buf = [0u8; FRAME_LEN];
    buf[0] = MAGIC;
    buf[1] = VERSION;
    buf[2] = TYPE_POSITION;
    buf[3] = PAYLOAD_LEN as u8;
    buf[4] = p.node_id;
    buf[5..9].copy_from_slice(&p.seq_nr.to_be_bytes());
    buf[9] = p.flags.0;
    buf[10..14].copy_from_slice(&p.lat_e7.to_be_bytes());
    buf[14..18].copy_from_slice(&p.lon_e7.to_be_bytes());
    buf[18..20].copy_from_slice(&p.alt_m.to_be_bytes());
    let crc = crc16_ccitt_false(&buf[..20]);
    buf[20..22].copy_from_slice(&crc.to_be_bytes());
    buf
}

pub fn decode_position(raw: &[u8]) -> Result<Position, FrameError> {
    if raw.len() != FRAME_LEN {
        return Err(FrameError::BadLength);
    }
    if raw[0] != MAGIC {
        return Err(FrameError::BadMagic);
    }
    if raw[1] != VERSION {
        return Err(FrameError::BadVersion);
    }
    if raw[2] != TYPE_POSITION {
        return Err(FrameError::UnknownType);
    }
    if raw[3] != PAYLOAD_LEN as u8 {
        return Err(FrameError::BadLength);
    }
    let wire_crc = u16::from_be_bytes([raw[20], raw[21]]);
    if crc16_ccitt_false(&raw[..20]) != wire_crc {
        return Err(FrameError::CrcMismatch);
    }
    let flags = Flags(raw[9]);
    if flags.has_reserved_bits() {
        return Err(FrameError::ReservedFlagBits);
    }
    let node_id = raw[4];
    let seq_nr = u32::from_be_bytes([raw[5], raw[6], raw[7], raw[8]]);
    let lat_e7 = i32::from_be_bytes([raw[10], raw[11], raw[12], raw[13]]);
    let lon_e7 = i32::from_be_bytes([raw[14], raw[15], raw[16], raw[17]]);
    let alt_m = i16::from_be_bytes([raw[18], raw[19]]);

    let lat_sentinel = lat_e7 == NO_FIX_LAT_LON_SENTINEL;
    let lon_sentinel = lon_e7 == NO_FIX_LAT_LON_SENTINEL;
    let alt_sentinel = alt_m == NO_FIX_ALT_SENTINEL;
    let all_sentinel = lat_sentinel && lon_sentinel && alt_sentinel;
    let any_sentinel = lat_sentinel || lon_sentinel || alt_sentinel;

    // GPS_VALID=1: no coordinate may be a sentinel value.
    // GPS_VALID=0: all three coordinates must be sentinel values.
    // Any partial sentinel combination is invalid.
    if flags.gps_valid() {
        if any_sentinel {
            return Err(FrameError::GpsValidSentinelMismatch);
        }
    } else if !all_sentinel {
        return Err(FrameError::GpsValidSentinelMismatch);
    }

    Ok(Position {
        node_id,
        seq_nr,
        flags,
        lat_e7,
        lon_e7,
        alt_m,
    })
}

pub fn packet_id(p: &Position) -> PacketId {
    PacketId {
        node_id: p.node_id,
        seq_nr: p.seq_nr,
    }
}
