use crate::frame::{DropReason, FrameError, RelayDecision, FRAME_LEN};
use crate::position::{decode_position, packet_id};
use crate::seen_cache::SeenCache;

/// Pure relay forwarding decision — no I/O, no hardware, no allocation.
///
/// Validation order per ARCHITECTURE.md §9 and ADR-013 §5:
///   1. Decode + validate frame (CRC, MAGIC, VER, TYPE, LEN, flags, sentinel consistency)
///   2. Check seen_cache for duplicate packet_id
///   3. Check for self-echo (node_id == my_node_id)
///   4. Insert into cache and return byte-identical frame for forwarding
pub fn relay_decide(
    raw: &[u8; FRAME_LEN],
    my_node_id: u8,
    now: u32,
    cache: &mut SeenCache,
) -> RelayDecision {
    let pos = match decode_position(raw.as_slice()) {
        Ok(p) => p,
        Err(FrameError::CrcMismatch) => return RelayDecision::Drop(DropReason::CrcFail),
        Err(FrameError::UnknownType | FrameError::BadVersion) => {
            return RelayDecision::Drop(DropReason::UnknownType)
        }
        Err(_) => return RelayDecision::Drop(DropReason::Malformed),
    };

    let pid = packet_id(&pos);

    if cache.contains(pid, now) {
        return RelayDecision::Drop(DropReason::Duplicate);
    }

    if pos.node_id == my_node_id {
        return RelayDecision::Drop(DropReason::SelfEcho);
    }

    cache.insert(pid, now);

    RelayDecision::EnqueueForward {
        packet_id: pid,
        frame_bytes: *raw,
    }
}
