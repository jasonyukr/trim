use std::io::{self, BufRead, BufWriter, Write};
use std::process;

fn main() {
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout);
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(ln) = line {
            if let Err(_) = writeln!(out, "{}", ln.trim()) {
                process::exit(1);
            }
        }
    }
    out.flush().unwrap();
}
