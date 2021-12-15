use std::{
    collections::HashMap,
    io::{self, Cursor, Read},
};

use super::ReadFrom;

pub struct DataReader {
    buff: Cursor<Vec<u8>>,
    size: usize,
}

pub type TlvMap = HashMap<u16, Vec<u8>>;

impl Read for DataReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let res = self.buff.read(buf)?;
        self.size -= res;
        Ok(res)
    }
}

impl DataReader {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            size: data.len(),
            buff: Cursor::new(data),
        }
    }
}

impl DataReader {
    pub fn read_data<T: ReadFrom>(&mut self) -> io::Result<T> {
        T::read_from(self)
    }

    pub fn read_data_short<T: ReadFrom>(&mut self) -> io::Result<T> {
        T::read_short_from(self)
    }

    pub fn read_data_limited<T: ReadFrom>(&mut self, size: usize) -> io::Result<T> {
        T::read_from_with_size(size, self)
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn read_available(self) -> Vec<u8> {
        self.buff.bytes().map(|b| b.unwrap()).collect()
    }

    pub fn read_tlv_map(&mut self, tag_size: usize) -> io::Result<TlvMap> {
        let mut map = TlvMap::new();
        loop {
            if self.len() < tag_size {
                break;
            }
            let k: u16 = if tag_size == 1 {
                self.read_data::<u8>()? as u16
            } else if tag_size == 2 {
                self.read_data()?
            } else if tag_size == 4 {
                self.read_data::<u32>()? as u16
            } else {
                0
            };

            if k == 255 {
                break;
            }

            let size = self.read_data::<u16>()? as usize;
            map.insert(k, self.read_data_limited::<Vec<u8>>(size)?);
        }
        Ok(map)
    }
}
