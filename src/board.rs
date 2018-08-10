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
                    $self.sizes[$self.column[(j - 325) as usize] as usize] $op 1;
                    j = $self.right[j as usize];
                }
                i = $self.down[i as usize];
            }
        }
    }
}

macro_rules! search_helper {
    ($self:ident, $r:ident, $dir:ident, $fn:ident) => {{
        let mut j = $self.$dir[$r as usize];
        for _i in 0..3 {
            let c = $self.column[(j - 325) as usize];
            $self.$fn(c);
            j = $self.$dir[j as usize];
        }
    }};
}

pub struct Board {
    up: [u16; 3241],
    down: [u16; 3241],
    left: [u16; 3241],
    right: [u16; 3241],
    column: [u16; 2916],
    sizes: [u8; 324],
    solution: [u16; 3241],
    solutions: u8,
    root: u16,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            up: [0; 3241],
            down: [0; 3241],
            left: [0; 3241],
            right: [0; 3241],
            column: [0; 2916],
            sizes: [9; 324],
            solution: [0; 3241],
            solutions: 0,
            root: 324,
        };
        board.horizontal();
        board.vertical();
        board
    }
    link!(self, link_down, down, up);
    link!(self, link_right, right, left);
    add_back!(self, add_back_row, right, left);
    add_back!(self, add_back_column, down, up);
    remove!(self, remove_from_column, down, up);
    remove!(self, remove_from_row, right, left);

    fn horizontal(&mut self) {
        let root = self.root;
        for c in 0..324 {
            self.link_right(c, c + 1);
        }
        self.link_right(323, root);
        self.link_right(root, 0);

        for r in (325..3241).step_by(4) {
            for c in 0..3 {
                self.link_right(r + c, r + c + 1);
            }
            self.link_right(r + 3, r);
        }
    }

    pub fn columns(&self, c: u16, d: u16) -> [u16; 4] {
        [
            c,
            c / 9 * 9 + d + 81,
            c % 9 * 9 + d + 162,
            (c / 3 - c / 9 * 3 + c / 27 * 3) * 9 + d + 243,
        ]
    }

    fn vertical(&mut self) {
        let mut boc: Vec<u16> = (0..324).collect();
        let start = 325;

        for c in 0..81 {
            for d in 0..9 {
                let v: u16 = 4 * (9 * c + d) + start;
                for i in 0..4 {
                    let cs = self.columns(c, d);
                    self.link_down(boc[cs[i] as usize], v + i as u16);
                    self.column[v as usize + i - 325] = cs[i];
                    boc[cs[i] as usize] = v + i as u16;
                }
            }
        }

        for c in 0..324 {
            self.link_down(boc[c as usize], c);
        }
    }

    fn choose(&self) -> u16 {
        let mut min_column = 0;
        let mut min_size = 10;
        let mut c = self.right[self.root as usize];
        while c != self.root {
            if self.sizes[c as usize] == 1 {
                return c;
            }
            if self.sizes[c as usize] < min_size {
                min_size = self.sizes[c as usize];
                min_column = c;
            }
            c = self.right[c as usize];
        }
        min_column
    }

    cover_method!(self, cover, remove_from_row, remove_from_column, -=);
    cover_method!(self, uncover, add_back_row, add_back_column, +=);

    pub fn search(&mut self, k: u32, p: &Fn([u16; 3241], u32)) {
        if self.solutions != 0 {
            return;
        }

        if self.right[self.root as usize] == self.root {
            p(self.solution, k);
            self.solutions += 1;
            return;
        }
        let c = self.choose();
        self.cover(c);
        let mut r = self.down[c as usize];
        while r != c {
            self.solution[k as usize] = r;
            search_helper!(self, r, right, cover);
            self.search(k + 1, p);
            search_helper!(self, r, left, uncover);
            r = self.down[r as usize];
        }
        self.uncover(c);
    }

    pub fn reset_solutions(&mut self) {
        self.solutions = 0;
    }
}
