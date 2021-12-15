use std::io::Write;

use byteorder::{LittleEndian, WriteBytesExt};

use crate::binary::WriteTo;

use super::{
    proto_msg::{DataEncoder, DynamicProtoMessage},
    VarInt, WriteToVarInt,
};

impl WriteToVarInt for bool {
    fn write_to_varint(&self, writer: &mut DataEncoder, key: u64) -> std::io::Result<()> {
        writer.uvarint(key | 0)?;
        let v = match self {
            true => 1,
            false => 0,
        };

        writer.uvarint(v)?;
        Ok(())
    }
}

impl WriteToVarInt for i32 {
    fn write_to_varint(&self, writer: &mut DataEncoder, key: u64) -> std::io::Result<()> {
        writer.uvarint(key | 0)?;
        writer.svarint(*self as i64)?;
        Ok(())
    }
}

impl WriteToVarInt for i64 {
    fn write_to_varint(&self, writer: &mut DataEncoder, key: u64) -> std::io::Result<()> {
        writer.uvarint(key | 0)?;
        writer.svarint(*self)?;
        Ok(())
    }
}

impl WriteToVarInt for u32 {
    fn write_to_varint(&self, writer: &mut DataEncoder, key: u64) -> std::io::Result<()> {
        writer.uvarint(key | 0)?;
        writer.uvarint(*self as u64)?;
        Ok(())
    }
}

impl WriteToVarInt for u64 {
    fn write_to_varint(&self, writer: &mut DataEncoder, key: u64) -> std::io::Result<()> {
        writer.uvarint(key | 0)?;
        writer.uvarint(*self)?;
        Ok(())
    }
}

impl WriteToVarInt for f32 {
    fn write_to_varint(&self, writer: &mut DataEncoder, key: u64) -> std::io::Result<()> {
        writer.uvarint(key | 5)?;
        writer.write_f32::<LittleEndian>(*self)?;
        Ok(())
    }
}

impl WriteToVarInt for f64 {
    fn write_to_varint(&self, writer: &mut DataEncoder, key: u64) -> std::io::Result<()> {
        writer.uvarint(key | 1)?;
        writer.write_f64::<LittleEndian>(*self)?;
        Ok(())
    }
}

impl WriteToVarInt for String {
    fn write_to_varint(&self, writer: &mut DataEncoder, key: u64) -> std::io::Result<()> {
        writer.uvarint(key | 2)?;
        let bytes = self.bytes().collect::<Vec<_>>();
        writer.uvarint(bytes.len() as u64)?;
        bytes.write_to(writer)
    }
}

impl WriteToVarInt for Vec<u64> {
    fn write_to_varint(&self, writer: &mut DataEncoder, key: u64) -> std::io::Result<()> {
        for i in self {
            i.write_to_varint(writer, key)?;
        }
        Ok(())
    }
}

impl WriteToVarInt for Vec<u8> {
    fn write_to_varint(&self, writer: &mut DataEncoder, key: u64) -> std::io::Result<()> {
        for i in self {
            writer.uvarint(key | 2)?;
            writer.uvarint((*i) as u64)?;
        }
        Ok(())
    }
}

impl WriteToVarInt for DynamicProtoMessage {
    fn write_to_varint(&self, writer: &mut DataEncoder, key: u64) -> std::io::Result<()> {
        writer.uvarint(key | 2)?;
        let b = self.encode()?;
        writer.uvarint(b.len() as u64)?;
        writer.write_all(&b)
    }
}
