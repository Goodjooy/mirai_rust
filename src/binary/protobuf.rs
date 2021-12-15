use std::io::{self, Write};

use self::proto_msg::DataEncoder;

mod proto_msg;
mod to_varint_impls;
pub trait VarInt: Write {
    fn uvarint(&mut self, data: u64) -> io::Result<usize>;
    fn svarint(&mut self, data: i64) -> io::Result<usize>;
}

impl<T: Write> VarInt for T {
    // 参考google 实现 https://go.dev/src/encoding/binary/varint.go?s=1611:1652#L41
    fn uvarint(&mut self, data: u64) -> io::Result<usize> {
        let mut idx = 0;
        let mut ld = data;
        while ld >= 0x80 {
            let t = (ld as u8) | 0x80;
            self.write_all(&[t])?;
            ld >>= 7;
            idx += 1;
        }

        Ok(idx + 1)
    }
    // 参考谷歌实现 https://go.dev/src/encoding/binary/varint.go?s=1611:1652#L83
    fn svarint(&mut self, data: i64) -> io::Result<usize> {
        let mut ux = (data as u64) << 1;
        if data < 0 {
            ux = !ux;
        }

        Self::uvarint(self, ux)
    }
}

pub trait WriteToVarInt

{
    fn write_to_varint(&self, writer: &mut DataEncoder, key: u64) -> io::Result<()>
    ;
}
