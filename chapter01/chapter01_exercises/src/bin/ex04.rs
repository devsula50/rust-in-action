use std::io::{self, Read, Write};

const BUFFSIZE: usize = 4096;

fn main() -> io::Result<()> {
    // Obtain standard input/output handles
    let stdin = io::stdin();
    let stdout = io::stdout();

    // Acquire a lock to ensure performance during repeated read/write operations
    let mut input = stdin.lock();
    let mut output = stdout.lock();

    let mut n: usize;
    let mut buf = [0u8; BUFFSIZE];

    loop {
        // read() returns the number of bytes read
        match input.read(&mut buf) {
            Ok(0) => break,     // Reaching EOF
            Ok(bytes_read) => {
                n = bytes_read;
                // write_all() guarantees writing the entire buffer
                if let Err(e) = output.write_all(&buf[..n]) {
                    eprintln!("write error: {}", e);
                    return Err(e);
                }
            }
            Err(e) => {
                eprintln!("read error: {}", e);
                return Err(e);
            }
        }
    }

    Ok(())
}
