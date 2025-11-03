use std::env;
use std::fs;
use std::io;

/// A function to print the contents of a directory
fn list_dir(path: &str) -> io::Result<()> {
    // read_dir returns a Result -> propagate errors using ?
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;     // Each entry is also a Result
        println!("{}", entry.file_name().to_string_lossy());
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Check the number of arguments
    if args.len() != 2 {
        // Instead of APUE-style err_quit, print to standard error + return a Result
        eprintln!("Usage: {} <directory>", args[0]);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid argument"));
    }
    
    let dir_name = &args[1];

    // Call list_dir, and propagate errors to the caller
    if let Err(e) = list_dir(dir_name) {
        eprintln!("Error reading directory '{}': {}", dir_name, e);
        return Err(e);
    }

    Ok(())
}
