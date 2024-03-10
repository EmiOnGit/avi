//! Outputs some metadata about a file.
//!
//! Usage: `cargo run --example avi_meta AVIFILE`

extern crate avirus;
use avirus::header::Header;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} AVIFILE", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    let header = Header::new(&path).unwrap();
    println!("header {}", header.height());
    println!("width {}", header.width());
}
