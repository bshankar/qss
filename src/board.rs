macro_rules! get {
    ($self: ident, $name: ident) => {
        pub fn $name(&$self, e: u16) -> u16 {
            $self.$name[e as usize]
        }
    }
}

macro_rules! set {
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
            let mut i = $self.down(c);
            while i != c {
                let j = $self.right(i);
                let k = $self.right(j);
                let l = $self.right(k);
                $self.column[j as usize] $op 1;
                $self.column[k as usize] $op 1;
                $self.column[l as usize] $op 1;
                $self.$fcol(j);
                $self.$fcol(k);
                $self.$fcol(l);
                i = $self.down(i);
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

    get!(self, up);
    get!(self, down);
    get!(self, left);
    get!(self, right);
    set!(self, set_up, up, down);
    set!(self, set_left, left, right);
    add_back!(self, add_back_row, up, down);
    add_back!(self, add_back_column, left, right);
    remove!(self, remove_from_column, up, down);
    remove!(self, remove_from_row, left, right);
    cover_method!(self, cover, remove_from_row, remove_from_column, -=);
    cover_method!(self, uncover, add_back_row, add_back_column, +=);

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
