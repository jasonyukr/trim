use std::io::{self, BufRead, BufWriter, Write};
use std::process;
use std::env;

fn main() {
    let trail = env::args().any(|arg| arg == "--trail");

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout);
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();

    if trail {
        let mut buf = String::new();
        while let Ok(n) = stdin_lock.read_line(&mut buf) {
            if n == 0 { break; }
            if writeln!(out, "{}", buf.trim_end()).is_err() {
                process::exit(1);
            }
            buf.clear();
        }
    } else {
        let mut buf = Vec::new();
        while let Ok(n) = stdin_lock.read_until(b'\n', &mut buf) {
            if n == 0 { break; }
            if out.write_all(&buf).is_err() {
                process::exit(1);
            }
            buf.clear();
        }
    }

    out.flush().unwrap();
}
