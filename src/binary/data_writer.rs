use std::io::Write;

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
