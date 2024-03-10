use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::Path,
};

use byteorder::{ByteOrder, LittleEndian};
use std::io::Result as IoResult;

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
    /// Total frames (including audioframes)
    pub fn total_frames(&self) -> u32 {
        let bytes = &self.value[6 * 4..7 * 4];
        LittleEndian::read_u32(bytes)
    }
    pub fn new<P: AsRef<Path>>(filename: P) -> IoResult<Self> {
        let mut f = BufReader::new(File::open(filename)?);
        let mut buf = [0u8; 4];
        let mut header_bytes = [0u8; HEADER_SIZE];
        f.seek(SeekFrom::Start(12))?;
        f.read_exact(&mut buf)?;
        while buf == *b"LIST" || buf == *b"JUNK" {
            f.read_exact(&mut buf)?;
            let s = LittleEndian::read_u32(&buf);
            f.read_exact(&mut buf)?;
            if buf == *b"hdrl" {
                f.read_exact(&mut header_bytes)?;
                return Ok(Header {
                    value: header_bytes,
                });
            }
            f.seek(SeekFrom::Current(i64::from(s) - 4))?;
            f.read_exact(&mut buf)?;
        }
        panic!("didn't find header");
    }
}
