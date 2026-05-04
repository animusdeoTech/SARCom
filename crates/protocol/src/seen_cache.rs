use crate::frame::{PacketId, SEEN_CACHE_CAPACITY, SEEN_CACHE_EXPIRY_SECONDS};

/// Fixed-capacity ring-buffer dedup cache, no_std, no heap.
///
/// When full, the oldest entry is evicted (circular overwrite semantics).
/// Timestamps are caller-supplied monotonic u32 seconds; entries expire after
/// SEEN_CACHE_EXPIRY_SECONDS (60 s).
pub struct SeenCache {
    entries: [Option<(PacketId, u32)>; SEEN_CACHE_CAPACITY],
    write_idx: usize,
}

impl SeenCache {
    pub const fn new() -> Self {
        Self {
            entries: [None; SEEN_CACHE_CAPACITY],
            write_idx: 0,
        }
    }

    /// Returns true if `pid` is present and its timestamp is within the expiry window.
    pub fn contains(&self, pid: PacketId, now: u32) -> bool {
        self.entries
            .iter()
            .flatten()
            .any(|(id, ts)| *id == pid && now.wrapping_sub(*ts) < SEEN_CACHE_EXPIRY_SECONDS)
    }

    /// Inserts `pid` at timestamp `now`.  When full, overwrites the oldest slot.
    pub fn insert(&mut self, pid: PacketId, now: u32) {
        self.entries[self.write_idx] = Some((pid, now));
        self.write_idx = (self.write_idx + 1) % SEEN_CACHE_CAPACITY;
    }
}

impl Default for SeenCache {
    fn default() -> Self {
        Self::new()
    }
}
