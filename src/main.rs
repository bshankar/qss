mod board;
use board::Board;

macro_rules! traverse {
    ($b:ident, $dir:ident, $s:expr) => {{
        let mut i = $b.$dir[$s as usize];
        while i != $s {
            println!("{}", i);
            i = $b.$dir[i as usize];
        }
    }};
}

fn main() {
    let mut b = Board::new();
    let start = 449;
    traverse!(b, right, start);
}
