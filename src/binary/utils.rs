use std::io::{self, Cursor, Read};

pub fn zlib_uncompress(src: Vec<u8>) -> io::Result<Vec<u8>> {
    let mut data = flate2::write::ZlibDecoder::new(Cursor::new(src));
    let mut res = Vec::with_capacity(1024);
    data.read_to_end(&mut res)?;
    Ok(res)
}

pub fn zlib_compress(data: Vec<u8>) -> io::Result<Vec<u8>> {
    let mut dst =
        flate2::write::ZlibEncoder::new(Cursor::new(data), flate2::Compression::default());
    let mut res = Vec::with_capacity(1024);
    dst.read_to_end(&mut res)?;
    Ok(res)
}

pub fn glib_uncompress(src: Vec<u8>) -> io::Result<Vec<u8>> {
    let mut data = flate2::write::GzDecoder::new(Cursor::new(src));
    let mut res = Vec::with_capacity(1024);
    data.read_to_end(&mut res)?;
    Ok(res)
}
pub fn glib_compress(data: Vec<u8>) -> io::Result<Vec<u8>> {
    let mut dst = flate2::write::GzEncoder::new(Cursor::new(data), flate2::Compression::default());
    let mut res = Vec::with_capacity(1024);
    dst.read_to_end(&mut res)?;
    Ok(res)
}
