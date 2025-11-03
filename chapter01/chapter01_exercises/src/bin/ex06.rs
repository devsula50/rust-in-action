use std::process;

fn main() {
    let pid = process::id();

    println!("pid = {pid}");
}