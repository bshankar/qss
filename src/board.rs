macro_rules! direction {
    ($self: ident, $name: ident) => {
        pub fn $name(&$self, e: u16) -> u16 {
            $self.$name[e as usize]
        }
    }
}

macro_rules! set_direction {
    ($self: ident, $name: ident, $dir: ident, $opp_dir: ident) => {
        fn $name(&mut $self, a: u16, b: u16) {
            $self.$dir[a as usize] = b;
            $self.$opp_dir[b as usize] = a;
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
        };
        board.horizontal();
        board.vertical();
        board
    }

    direction!(self, up);
    direction!(self, down);
    direction!(self, left);
    direction!(self, right);
    set_direction!(self, set_up, up, down);
    set_direction!(self, set_left, left, right);

    fn horizontal(&mut self) {
        for c in 0..323 {
            self.set_left(c + 1, c);
        }
        self.set_left(0, 323);

        for r in (324..1053).step_by(4) {
            for c in 0..4 {
                self.set_left(r + c + 1, r + c);
            }
            self.set_left(r, r + 3);
        }
    }

    fn vertical(&mut self) {
        let mut boc: Vec<u16> = (0..324).collect();
        let start = 324;

        for c in 0..81 {
            for d in 0..9 {
                let v = 4 * (9 * c + d) + start;
                self.set_up(v, boc[c as usize]);
                self.set_up(v + 1, boc[(c / 9 * 9 + d + 81) as usize]);
                self.set_up(v + 2, boc[(c % 9 * 9 + d + 162) as usize]);
                self.set_up(
                    v + 3,
                    boc[((c / 3 - c / 9 * 3 + c / 27 * 3) * 9 + d + 243) as usize],
                );

                boc[c as usize] = v;
                boc[(c / 9 * 9 + d + 81) as usize] = v + 1;
                boc[(c % 9 * 9 + d + 162) as usize] = v + 2;
                boc[((c / 3 - c / 9 * 3 + c / 27 * 3) * 9 + d + 243) as usize] = v + 3;
            }
        }

        for c in 0..324 {
            self.set_up(c, boc[c as usize]);
        }
    }
}
