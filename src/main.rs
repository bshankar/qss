mod board;
mod solve;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut b = board::Board::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        solve::solve(&line?, &mut b);
    }
    Ok(())
}
