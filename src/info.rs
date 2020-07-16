use std::io::{Read, Error};
use serde::Serialize;
use byteorder::{ReadBytesExt, LittleEndian};


/// Reference to structure description
/// https://github.com/vitalif/ntfs-3g/blob/04d4b37a9a6c992d89c93193d0abdd13ab1f2931/ntfsprogs/mkntfs.c#L168
/// 
#[derive(Serialize, Debug)]
pub struct UpcaseInfo {
    pub len: u32,
    pub filler: u32,
    pub crc: u64,
    pub osmajor: u32,
    pub osminor: u32,
    pub build: u32,
    pub packmajor: u16,
    pub packminor: u16,
}

impl UpcaseInfo {
    pub fn new<R: Read>(mut buffer: R) -> Result<UpcaseInfo, Error> {
        let len = buffer.read_u32::<LittleEndian>()?;
        let filler = buffer.read_u32::<LittleEndian>()?;
        let crc = buffer.read_u64::<LittleEndian>()?;
        let osmajor = buffer.read_u32::<LittleEndian>()?;
        let osminor = buffer.read_u32::<LittleEndian>()?;
        let build = buffer.read_u32::<LittleEndian>()?;
        let packmajor = buffer.read_u16::<LittleEndian>()?;
        let packminor = buffer.read_u16::<LittleEndian>()?;

        Ok(
            UpcaseInfo { 
                len, filler, crc, osmajor, osminor,
                build, packmajor, packminor
            }
        )
    }
}

impl std::fmt::Display for UpcaseInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, 
            "crc: {}\nosmajor: {}\nosminor: {}\nbuild: {}\npackmajor: {}\npackminor: {}", 
            self.crc, self.osmajor, self.osminor, self.build, self.packmajor, self.packminor
        )
    }
}