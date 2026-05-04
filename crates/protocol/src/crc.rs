/// CRC-16/CCITT-FALSE: poly=0x1021, init=0xFFFF, RefIn=false, RefOut=false, XorOut=0x0000.
/// Check value for ASCII "123456789" is 0x29B1.
pub fn crc16_ccitt_false(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for &byte in data {
        crc ^= (byte as u16) << 8;
        for _ in 0..8 {
            crc = if crc & 0x8000 != 0 {
                (crc << 1) ^ 0x1021
            } else {
                crc << 1
            };
        }
    }
    crc
}
