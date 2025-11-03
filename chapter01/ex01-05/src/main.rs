use std::io::{self, Read, Write};
use std::io::{BufReader, BufWriter};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();

    // Wrap stdin/stdout in buffered reader/writer
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());

    let mut buf = [0u8; 1];

    // Read one byte at a time
    while let Ok(n) = reader.read(&mut buf) {
        if n == 0 {
            break;      // EOF
        }
        writer.write_all(&buf[..n])?;
    }

    writer.flush()?;        // ensure buffered output is written
    Ok(())
}