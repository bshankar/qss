mod board;
use board::Board;

fn main() {
    let b = Board::new();
    println!("{}", b.down(b.up(0)));
}
