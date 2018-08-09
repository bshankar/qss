macro_rules! link {
    ($self: ident, $name: ident, $dir: ident, $opp_dir: ident) => {
        fn $name(&mut $self, a: u16, b: u16) {
            $self.$dir[a as usize] = b;
            $self.$opp_dir[b as usize] = a;
        }
    }
}

macro_rules! remove {
    ($self: ident, $name: ident, $dir: ident, $opp_dir: ident) => {
        fn $name(&mut $self, a: u16) {
            $self.$opp_dir[$self.$dir[a as usize] as usize] = $self.$opp_dir[a as usize];
            $self.$dir[$self.$opp_dir[a as usize] as usize] = $self.$dir[a as usize];
        }
    }
}

macro_rules! add_back {
    ($self: ident, $name: ident, $dir: ident, $opp_dir: ident) => {
        fn $name(&mut $self, a: u16) {
            $self.$opp_dir[$self.$dir[a as usize] as usize] = a;
            $self.$dir[$self.$opp_dir[a as usize] as usize] = a;
        }
    }
}

macro_rules! cover_method {
    ($self: ident, $name: ident, $frow: ident, $fcol: ident, $op: tt) => {
        pub fn $name(&mut $self, c: u16) {
            $self.$frow(c);
            let mut i = $self.down[c as usize];
            while i != c {
                let mut j = $self.right[i as usize];
                for _k in 0..3 {
                    $self.$fcol(j);
                    $self.column[j as usize] $op 1;
                    j = $self.right[j as usize];
                }
                i = $self.down[i as usize];
            }
        }
    }
}

pub struct Board {
    up: [u16; 3240],
    down: [u16; 3240],
    left: [u16; 3240],
    right: [u16; 3240],
    column: [u16; 2916],
    sizes: [u8; 324],
    root: u16,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            up: [0; 3240],
            down: [0; 3240],
            left: [0; 3240],
            right: [0; 3240],
            column: [0; 2916],
            sizes: [9; 324],
            root: 324,
        };
        board.horizontal();
        board.vertical();
        board
    }
    link!(self, link_down, down, up);
    link!(self, link_right, right, left);
    add_back!(self, add_back_row, up, down);
    add_back!(self, add_back_column, left, right);
    remove!(self, remove_from_column, up, down);
    remove!(self, remove_from_row, left, right);
    cover_method!(self, cover, remove_from_row, remove_from_column, -=);
    cover_method!(self, uncover, add_back_row, add_back_column, +=);

    fn horizontal(&mut self) {
        let root = self.root;
        for c in 0..324 {
            self.link_right(c, c + 1);
        }
        self.link_right(324, root);
        self.link_right(root, 0);

        for r in (325..1054).step_by(4) {
            for c in 0..4 {
                self.link_right(r + c, r + c + 1);
            }
            self.link_right(r + 3, r);
        }
    }

    fn vertical(&mut self) {
        let mut boc: Vec<u16> = (0..324).collect();
        let start = 325;

        for c in 0..81 {
            for d in 0..9 {
                let v = 4 * (9 * c + d) + start;
                let cs = [
                    c,
                    c / 9 * 9 + d + 81,
                    c % 9 * 9 + d + 162,
                    (c / 3 - c / 9 * 3 + c / 27 * 3) * 9 + d + 243,
                ];

                for i in 0..4 {
                    self.link_down(boc[cs[i]], (v + i) as u16);
                    boc[cs[i]] = (v + i) as u16;
                }
            }
        }

        for c in 0..324 {
            self.link_down(boc[c as usize], c);
        }
    }
}
