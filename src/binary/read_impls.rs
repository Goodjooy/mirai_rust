use byteorder::{BigEndian, ReadBytesExt};

use super::ReadFrom;

impl ReadFrom for Vec<u8> {
    fn read_from<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        Self::read_short_from(reader)
    }
    fn read_from_with_size<R: std::io::Read>(size: usize, reader: &mut R) -> std::io::Result<Self> {
        let mut tmp = vec![0u8; size];
        reader.read_exact(&mut tmp)?;
        Ok(tmp)
    }
    fn read_short_from<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let size = u16::read_from(reader)? as usize;
        Self::read_from_with_size(size, reader)
    }
}

impl ReadFrom for u8 {
    fn read_from<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        reader.read_u8()
    }
}

impl ReadFrom for u16 {
    fn read_from<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        reader.read_u16::<BigEndian>()
    }
}

impl ReadFrom for u32 {
    fn read_from<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        reader.read_u32::<BigEndian>()
    }
}

impl ReadFrom for i32 {
    fn read_from<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        reader.read_i32::<BigEndian>()
    }
}

impl ReadFrom for u64 {
    fn read_from<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        reader.read_u64::<BigEndian>()
    }
}

impl ReadFrom for i64 {
    fn read_from<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        reader.read_i64::<BigEndian>()
    }
}

impl ReadFrom for String {
    fn read_from<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let size = (reader.read_i32::<BigEndian>()? - 4) as usize;
        let mut tmp = vec![0u8; size];
        reader.read_exact(&mut tmp)?;
        Ok(String::from_utf8_lossy(&tmp).to_string())
    }
    fn read_from_with_size<R: std::io::Read>(size: usize, reader: &mut R) -> std::io::Result<Self> {
        let mut tmp = vec![0u8; size];
        reader.read_exact(&mut tmp)?;
        Ok(String::from_utf8_lossy(&tmp).to_string())
    }

    fn read_short_from<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let size = (reader.read_u16::<BigEndian>()?) as usize;
        let mut tmp = vec![0u8; size];
        reader.read_exact(&mut tmp)?;
        Ok(String::from_utf8_lossy(&tmp).to_string())
    }
}
