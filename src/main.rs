use std::io::{self, BufRead, BufWriter, Write};

fn main() {
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout);
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(ln) = line {
            writeln!(out, "{}", ln.trim()).unwrap();
        }
    }
    out.flush().unwrap();
}
