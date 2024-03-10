use byteorder::{ByteOrder, LittleEndian};

pub const HEADER_SIZE: usize = 16 * 4;
pub struct Header {
    pub value: [u8; HEADER_SIZE],
}
impl Header {
    /// height in pixel
    pub fn height(&self) -> u32 {
        let bytes = &self.value[11 * 4..12 * 4];
        LittleEndian::read_u32(bytes)
    }
    /// width in pixel
    pub fn width(&self) -> u32 {
        let bytes = &self.value[10 * 4..11 * 4];
        LittleEndian::read_u32(bytes)
    }
}
