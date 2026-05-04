pub const FRAME_LEN: usize = 22;
pub const PAYLOAD_LEN: usize = 16;
pub const MAGIC: u8 = 0xA5;
pub const VERSION: u8 = 0x01;
pub const TYPE_POSITION: u8 = 0x01;
pub const SEEN_CACHE_CAPACITY: usize = 32;
pub const SEEN_CACHE_EXPIRY_SECONDS: u32 = 60;
pub const NO_FIX_LAT_LON_SENTINEL: i32 = 0x7FFF_FFFF;
pub const NO_FIX_ALT_SENTINEL: i16 = 0x7FFF;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PacketId {
    pub node_id: u8,
    pub seq_nr: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameError {
    BadMagic,
    BadVersion,
    UnknownType,
    BadLength,
    CrcMismatch,
    ReservedFlagBits,
    GpsValidSentinelMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropReason {
    CrcFail,
    UnknownType,
    Duplicate,
    SelfEcho,
    Malformed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelayDecision {
    Drop(DropReason),
    EnqueueForward {
        packet_id: PacketId,
        frame_bytes: [u8; FRAME_LEN],
    },
}
