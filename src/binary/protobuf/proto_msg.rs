use std::{
    collections::HashMap,
    io::{self, Write},
};

use super::WriteToVarInt;

pub struct DynamicProtoMessage(HashMap<u64, Box<dyn WriteToVarInt>>);

impl DynamicProtoMessage {
    pub fn encode(&self) -> io::Result<Vec<u8>> {
        let mut encoder = DataEncoder {
            buff: Vec::with_capacity(1024),
        };

        for (key, v) in &self.0 {
            v.write_to_varint(&mut encoder, *key)?;
        }

        Ok(encoder.buff)
    }
}

pub struct DataEncoder {
    buff: Vec<u8>,
}

impl Write for DataEncoder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buff.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buff.flush()
    }
}
