#[derive(Debug)]
pub struct Bitboard {
    board: Vec<u128>,
    header: Vec<usize>,
    rows: usize,
}

impl Bitboard {
    pub fn new(rows: usize, columns: usize) -> Self {
        Bitboard {
            board: vec![0; (rows * columns + 127) / 128],
            header: vec![0; columns],
            rows: rows,
        }
    }

    pub fn init(&mut self, f: &Fn(usize, usize) -> Option<bool>) {
        for r in 0..self.rows {
            for c in 0..self.header.len() {
                if f(r, c) == Some(true) {
                    self.set(r, c)
                }
            }
        }
    }

    pub fn set(&mut self, row: usize, column: usize) {
        let bit_index = self.header.len() * row + column;
        self.board[bit_index / 128] |= 1 << (bit_index % 128);
    }

    pub fn unset(&mut self, row: usize, column: usize) {
        let bit_index = self.header.len() * row + column;
        self.board[bit_index / 128] &= !(1 << (bit_index % 128));
    }

    pub fn min(&self) -> usize {
        let mut min_column = 0;
        let mut min_size = self.rows + 1;
        for i in 0..self.header.len() {
            let size = self.header[i];
            if size < min_size {
                min_column = i;
                min_size = size;
            }
        }
        min_column
    }

    // cover, uncover
}
