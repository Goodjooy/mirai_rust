use byteorder::{BigEndian, WriteBytesExt};

use super::WriteTo;

impl WriteTo for [u8] {
    fn write_to<W: std::io::Write>(&self, write: &mut W) -> std::io::Result<()> {
        write.write_all(self)
    }

    fn write_short_to<W: std::io::Write>(&self, write: &mut W) -> std::io::Result<()> {
        ((self.len()) as u16).write_to(write)?;
        self.write_to(write)
    }
}

impl WriteTo for &[u8] {
    fn write_to<W: std::io::Write>(&self, write: &mut W) -> std::io::Result<()> {
        write.write_all(self)
    }

    fn write_short_to<W: std::io::Write>(&self, write: &mut W) -> std::io::Result<()> {
        ((self.len()) as u16).write_to(write)?;
        self.write_to(write)
    }
}

impl WriteTo for Vec<u8> {
    fn write_to<W: std::io::Write>(&self, write: &mut W) -> std::io::Result<()> {
        write.write_all(self)
    }

    fn write_short_to<W: std::io::Write>(&self, write: &mut W) -> std::io::Result<()> {
        ((self.len()) as u16).write_to(write)?;
        self.write_to(write)
    }
}

impl WriteTo for u8 {
    fn write_to<W: std::io::Write>(&self, write: &mut W) -> std::io::Result<()> {
        write.write_all(&[*self])
    }
}

impl WriteTo for u16 {
    fn write_to<W: std::io::Write>(&self, write: &mut W) -> std::io::Result<()> {
        write.write_u16::<BigEndian>(*self)
    }
}

impl WriteTo for u32 {
    fn write_to<W: std::io::Write>(&self, write: &mut W) -> std::io::Result<()> {
        write.write_u32::<BigEndian>(*self)
    }
}

impl WriteTo for u64 {
    fn write_to<W: std::io::Write>(&self, write: &mut W) -> std::io::Result<()> {
        write.write_u64::<BigEndian>(*self)
    }
}
impl<'a> WriteTo for &'a str {
    fn write_to<W: std::io::Write>(&self, write: &mut W) -> std::io::Result<()> {
        let payload = self.bytes().collect::<Vec<_>>();
        write.write_u32::<BigEndian>(payload.len() as u32 + 4)?;
        write.write_all(&payload)
    }
    fn write_short_to<W: std::io::Write>(&self, write: &mut W) -> std::io::Result<()> {
        let payload = self.bytes().collect::<Vec<_>>();
        payload.as_slice().write_short_to(write)
    }
}

impl WriteTo for String {
    fn write_to<W: std::io::Write>(&self, write: &mut W) -> std::io::Result<()> {
        let payload = self.bytes().collect::<Vec<_>>();
        write.write_u32::<BigEndian>(payload.len() as u32 + 4)?;
        write.write_all(&payload)
    }
    fn write_short_to<W: std::io::Write>(&self, write: &mut W) -> std::io::Result<()> {
        let payload = self.bytes().collect::<Vec<_>>();
        payload.as_slice().write_short_to(write)
    }
}

impl WriteTo for bool {
    fn write_to<W: std::io::Write>(&self, write: &mut W) -> std::io::Result<()> {
        match self {
            true => 0x01u8,
            false => 0x00,
        }
        .write_to(write)
    }
}
