/// PC-only fake-radio integration test.
/// No hardware, no SPI, no lora-phy.  Proves the encode → relay → decode pipeline.
use protocol::{
    decode_position, encode_position, relay_decide, DropReason, Flags, PacketId, Position,
    RelayDecision, SeenCache, FRAME_LEN,
};

fn make_pos(node_id: u8, seq_nr: u32) -> Position {
    Position {
        node_id,
        seq_nr,
        flags: Flags(Flags::GPS_VALID_MASK),
        lat_e7: 471_234_567,
        lon_e7: 135_678_901,
        alt_m: 1847,
    }
}

/// tag → relay → gateway: one valid POSITION, forwarded byte-identically.
#[test]
fn tag_relay_gateway_single_hop() {
    let tag_pos = make_pos(1, 1);
    let tag_frame: [u8; FRAME_LEN] = encode_position(tag_pos);

    let mut cache = SeenCache::new();
    let decision = relay_decide(
        &tag_frame, /*my_node_id=*/ 2, /*now=*/ 0, &mut cache,
    );

    let fwd_frame = match decision {
        RelayDecision::EnqueueForward { frame_bytes, .. } => frame_bytes,
        other => panic!("expected EnqueueForward, got {other:?}"),
    };

    // Relay must forward byte-identically
    assert_eq!(fwd_frame, tag_frame, "frame must be byte-identical");

    // Gateway decodes the forwarded frame and gets back the original position
    let gw_pos = decode_position(&fwd_frame).expect("gateway decode must succeed");
    assert_eq!(gw_pos, tag_pos);
}

/// Five copies of the same frame hit the relay; only the first produces a forward.
#[test]
fn duplicate_storm_single_forward() {
    let frame: [u8; FRAME_LEN] = encode_position(make_pos(1, 10));
    let mut cache = SeenCache::new();

    let first = relay_decide(&frame, 2, 0, &mut cache);
    assert!(
        matches!(first, RelayDecision::EnqueueForward { .. }),
        "first copy must be forwarded"
    );

    for i in 1..5u32 {
        let dup = relay_decide(&frame, 2, i, &mut cache);
        assert_eq!(
            dup,
            RelayDecision::Drop(DropReason::Duplicate),
            "copy {i} must be dropped as duplicate"
        );
    }
}

/// relay_decide drops its own emitted packet (self-echo).
#[test]
fn relay_drops_self_echo() {
    let frame: [u8; FRAME_LEN] = encode_position(make_pos(2, 5));
    let mut cache = SeenCache::new();
    let decision = relay_decide(&frame, /*my_node_id=*/ 2, 0, &mut cache);
    assert_eq!(decision, RelayDecision::Drop(DropReason::SelfEcho));
}

/// A seen_cache entry expires after 60 s; the same packet_id is forwarded again afterwards.
#[test]
fn seen_cache_expires_after_60s() {
    let frame: [u8; FRAME_LEN] = encode_position(make_pos(1, 99));
    let mut cache = SeenCache::new();

    // First reception at t=0: forward
    assert!(matches!(
        relay_decide(&frame, 2, 0, &mut cache),
        RelayDecision::EnqueueForward { .. }
    ));

    // t=59: still cached → duplicate
    assert_eq!(
        relay_decide(&frame, 2, 59, &mut cache),
        RelayDecision::Drop(DropReason::Duplicate)
    );

    // t=60: entry has expired → forward again
    assert!(
        matches!(
            relay_decide(&frame, 2, 60, &mut cache),
            RelayDecision::EnqueueForward { .. }
        ),
        "entry must have expired at t=60"
    );
}

/// When SeenCache is full (32 entries), inserting a 33rd evicts the oldest entry.
#[test]
fn seen_cache_evicts_oldest_when_full() {
    let mut cache = SeenCache::new();

    // Fill the 32-entry cache with seq=0..31 at t=0 (write_idx wraps back to 0).
    for seq in 0u32..32 {
        let frame: [u8; FRAME_LEN] = encode_position(make_pos(1, seq));
        assert!(
            matches!(
                relay_decide(&frame, 2, 0, &mut cache),
                RelayDecision::EnqueueForward { .. }
            ),
            "seq {seq} should forward"
        );
    }

    // Verify all 32 are present (no inserts here — all return Drop, write_idx stays 0).
    for seq in 0u32..32 {
        let frame: [u8; FRAME_LEN] = encode_position(make_pos(1, seq));
        assert_eq!(
            relay_decide(&frame, 2, 0, &mut cache),
            RelayDecision::Drop(DropReason::Duplicate),
            "seq {seq} should be cached"
        );
    }

    // Insert seq=32: cache full, so entries[0] (seq=0) is overwritten. write_idx → 1.
    let frame_32: [u8; FRAME_LEN] = encode_position(make_pos(1, 32));
    assert!(
        matches!(
            relay_decide(&frame_32, 2, 0, &mut cache),
            RelayDecision::EnqueueForward { .. }
        ),
        "seq=32 must be forwarded (evicts oldest slot)"
    );

    // seq=0 is gone — relay forwards it again. This re-insert places seq=0 at entries[1],
    // overwriting seq=1. write_idx → 2.
    let frame_0: [u8; FRAME_LEN] = encode_position(make_pos(1, 0));
    assert!(
        matches!(
            relay_decide(&frame_0, 2, 0, &mut cache),
            RelayDecision::EnqueueForward { .. }
        ),
        "seq=0 must have been evicted and now forwards again"
    );

    // After two eviction events: seq=0 and seq=1 were overwritten (at indices 0 and 1).
    // seq=2..31, seq=32, and the re-inserted seq=0 are present.
    // Spot-check entries that are definitely still cached.
    for seq in [2u32, 10, 20, 31, 32] {
        let frame: [u8; FRAME_LEN] = encode_position(make_pos(1, seq));
        assert_eq!(
            relay_decide(&frame, 2, 0, &mut cache),
            RelayDecision::Drop(DropReason::Duplicate),
            "seq {seq} should still be cached"
        );
    }
}

/// Packet with bad CRC is dropped as CrcFail.
#[test]
fn relay_drops_bad_crc() {
    let mut frame: [u8; FRAME_LEN] = encode_position(make_pos(1, 200));
    frame[21] ^= 0xFF;
    let mut cache = SeenCache::new();
    assert_eq!(
        relay_decide(&frame, 2, 0, &mut cache),
        RelayDecision::Drop(DropReason::CrcFail)
    );
}

/// Packet with unknown TYPE byte is dropped as UnknownType.
#[test]
fn relay_drops_unknown_type() {
    let mut frame: [u8; FRAME_LEN] = encode_position(make_pos(1, 201));
    frame[2] = 0x42; // unknown TYPE
    let mut cache = SeenCache::new();
    assert_eq!(
        relay_decide(&frame, 2, 0, &mut cache),
        RelayDecision::Drop(DropReason::UnknownType)
    );
}

/// PacketId is correctly derived from node_id and seq_nr.
#[test]
fn packet_id_fields() {
    let pos = make_pos(7, 12345);
    let pid = protocol::packet_id(&pos);
    assert_eq!(
        pid,
        PacketId {
            node_id: 7,
            seq_nr: 12345
        }
    );
}

/// relay_decide rejects a frame shorter than FRAME_LEN as Malformed.
#[test]
fn relay_drops_short_frame() {
    let short = [0xA5u8; 21]; // one byte too short
    let mut cache = SeenCache::new();
    assert_eq!(
        relay_decide(&short, 2, 0, &mut cache),
        RelayDecision::Drop(DropReason::Malformed)
    );
}

/// relay_decide rejects a frame longer than FRAME_LEN as Malformed.
#[test]
fn relay_drops_long_frame() {
    let long = [0xA5u8; 23]; // one byte too long
    let mut cache = SeenCache::new();
    assert_eq!(
        relay_decide(&long, 2, 0, &mut cache),
        RelayDecision::Drop(DropReason::Malformed)
    );
}
