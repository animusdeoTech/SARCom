#![no_std]

pub mod crc;
pub mod flags;
pub mod frame;
pub mod position;
pub mod relay;
pub mod seen_cache;

pub use crc::crc16_ccitt_false;
pub use flags::Flags;
pub use frame::{
    DropReason, FrameError, PacketId, RelayDecision, FRAME_LEN, MAGIC, NO_FIX_ALT_SENTINEL,
    NO_FIX_LAT_LON_SENTINEL, PAYLOAD_LEN, SEEN_CACHE_CAPACITY, SEEN_CACHE_EXPIRY_SECONDS,
    TYPE_POSITION, VERSION,
};
pub use position::{decode_position, encode_position, packet_id, Position};
pub use relay::relay_decide;
pub use seen_cache::SeenCache;
