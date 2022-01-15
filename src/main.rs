use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for ln in stdin.lock().lines() {
        let line;
        match ln {
            Ok(data) => line = data,
            Err(_) => continue
        }
        println!("{}", line.trim());
    }
}
