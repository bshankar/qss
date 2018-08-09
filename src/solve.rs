use board::Board;

fn empty(ch: char) -> bool {
    ch == '0' || ch == '.'
}

pub fn print_solution(s: [u16; 3241], k: u32, givens: Vec<char>, silent: bool) {
    let mut solution: Vec<char> = givens;
    for i in 0..k {
        let r = (s[i as usize] - 325) / 4;
        let cell = r / 9;
        let digit = (r % 9) as u8;
        solution[cell as usize] = (digit + 49) as char;
    }
    let sol: String = solution.iter().collect();
    if !silent {
        println!("{}", sol);
    }
}

fn prepare(s: &String, b: &mut Board) -> Vec<u16> {
    let mut covered = Vec::with_capacity(324);
    for c in 0..s.len() {
        let ch = s.as_bytes()[c] as char;
        if !empty(ch) {
            let cs = b.columns(c as u16, ch as u16 - 49);
            for &col in cs.iter() {
                b.cover(col);
                covered.push(col);
            }
        }
    }
    covered
}

// fn reset(s: , b: ) {}

pub fn solve(s: &String, b: &mut Board, silent: bool) {
    let cs = prepare(s, b);
    b.search(0, &|sol, k| {
        print_solution(sol, k, s.chars().collect(), silent)
    });
}

// fn bench(s: ) {}
