mod board;
use board::Bitboard;

fn main() {
    let mut bb = Bitboard::new(3, 4);
    bb.set(0, 0);
    bb.set(0, 1);
    bb.set(3, 0);
    bb.unset(3, 0);
    bb.unset(0, 1);
    println!("{:?}", bb);
}
