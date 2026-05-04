#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Flags(pub u8);

impl Flags {
    pub const GPS_VALID_MASK: u8 = 0x01;
    pub const SOS_MASK: u8 = 0x02;
    pub const BATT_LOW_MASK: u8 = 0x04;
    const RESERVED_MASK: u8 = 0xF8;

    pub fn gps_valid(self) -> bool {
        self.0 & Self::GPS_VALID_MASK != 0
    }

    pub fn sos(self) -> bool {
        self.0 & Self::SOS_MASK != 0
    }

    pub fn batt_low(self) -> bool {
        self.0 & Self::BATT_LOW_MASK != 0
    }

    pub fn has_reserved_bits(self) -> bool {
        self.0 & Self::RESERVED_MASK != 0
    }
}
