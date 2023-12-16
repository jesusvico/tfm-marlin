use std::io::{self, Write};

pub struct InterceptingWriter {
    inner: io::Stdout,
}

impl InterceptingWriter {
    pub fn new() -> Self {
        Self { inner: io::stdout() }
    }
}

impl Write for InterceptingWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Do something with the data, e.g., print it before forwarding
        println!("Intercepted: {}", String::from_utf8_lossy(buf));

        // Forward the data to the original stdout
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}