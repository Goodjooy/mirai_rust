use std::io::{self, Write};

use super::{tea::Tea, WriteTo};

struct DataWriter {
    buff: Vec<u8>,
}

impl Write for DataWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buff.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buff.flush()
    }
}

impl WriteTo for DataWriter {
    fn write_to<W: Write>(&self, write: &mut W) -> io::Result<()> {
        self.buff.write_to(write)
    }
}

impl DataWriter {
    pub fn new_filled<F>(mut func: F) -> Vec<u8>
    where
        F: FnMut(&mut Self),
    {
        let mut s = Self {
            buff: Vec::with_capacity(1024),
        };
        func(&mut s);
        s.buff
    }

    pub fn new<F>(mut func: F) -> Self
    where
        F: FnMut(&mut Self),
    {
        let mut s = Self {
            buff: Vec::with_capacity(1024),
        };
        func(&mut s);
        s
    }
    pub fn write_data<T: WriteTo>(&mut self, data: T) -> io::Result<()> {
        data.write_to(self)
    }
    pub fn write_short_data<T: WriteTo>(&mut self, data: T) -> io::Result<()> {
        data.write_short_to(self)
    }

    pub fn encrypted_write(&mut self, key: &[u8], data: &[u8]) -> io::Result<()> {
        let tea = Tea::new(key);
        let src = tea.encrypt(data).expect("Failure to Encrypt");
        src.write_to(self)
    }

    pub fn write_in_tlv_package<F: FnMut(&mut Self)>(
        &mut self,
        offset: usize,
        func: F,
    ) -> io::Result<()> {
        let tw = Self::new(func).buff;
        ((tw.len() + offset) as u32).write_to(self)?;
        tw.write_to(self)
    }

    pub fn write_in_uni_package(
        &mut self,
        command_name: &str,
        session_id: &[u8],
        extra_data: &[u8],
        body: &[u8],
    ) -> io::Result<()> {
        let mut wt = Self::new(|_| {});
        wt.write_data(command_name)?;
        wt.write_data(8u32)?;
        wt.write_data(session_id)?;

        if extra_data.len() == 0 {
            wt.write_data(0x04u32)?;
        } else {
            wt.write_data((extra_data.len() + 4) as u32)?;
            wt.write_data(extra_data)?;
        }
        self.write_data((wt.buff.len() + 4) as u32)?;
        self.write_data(wt)?;

        self.write_data((body.len() + 4) as u32)?;
        self.write_data(body)
    }

    pub fn write_tlv_limited_size(&mut self, data: &[u8], limit: usize) -> io::Result<()> {
        if data.len() <= limit {
            self.write_short_data(data)
        } else {
            self.write_short_data(&data[0..limit])
        }
    }
}
