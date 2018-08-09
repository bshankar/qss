mod board;
mod solve;

fn main() {
    let mut b = board::Board::new();
    solve::solve(
        &"000000010400000000020000000000050407008000300001090000300400200050100000000806000"
            .to_string(),
        &mut b,
        false,
    );
}
