use std::io::{self, Read, Write};

pub mod data_writer;
pub mod data_reader;
pub mod write_impls;
pub mod read_impls;
pub mod protobuf;

mod utils;

pub trait WriteTo {
    fn write_to<W: Write>(&self, write: &mut W) -> io::Result<()>;
    fn write_short_to<W: Write>(&self, write: &mut W) -> io::Result<()> {
        Self::write_to(&self, write)
    }
}

pub trait ReadFrom: Sized {
    fn read_from<R: Read>(reader: &mut R) -> io::Result<Self>;
    fn read_from_with_size<R: Read>(_size: usize, reader: &mut R) -> io::Result<Self> {
        Self::read_from(reader)
    }
    fn read_short_from<R: Read>(reader: &mut R) -> io::Result<Self> {
        Self::read_from(reader)
    }
}
