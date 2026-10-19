use std::io::{self, Read, Write};
use std::str::{FromStr, SplitAsciiWhitespace};

const _MAX_N: usize = 2e5 as usize + 10;

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

fn solution(tokens: &mut SplitAsciiWhitespace, out: &mut impl Write) {
    let _n: usize = next(tokens);
    let a: Vec<char> = tokens.next().unwrap().chars().collect();
    let b: Vec<char> = tokens.next().unwrap().chars().collect();

    let a_even_1 = a.iter().step_by(2).filter(|&&ch| ch == '1').count();
    let a_odd_1 = a.iter().skip(1).step_by(2).filter(|&&ch| ch == '1').count();
    let b_even_1 = b.iter().step_by(2).filter(|&&ch| ch == '1').count();
    let b_odd_1 = b.iter().skip(1).step_by(2).filter(|&&ch| ch == '1').count();

    if a_even_1 == b_even_1 && a_odd_1 == b_odd_1 {
        writeln!(out, "YES").unwrap();
    } else {
        writeln!(out, "NO").unwrap();
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());

    let t: usize = next(&mut tokens);
    for _ in 0..t {
        solution(&mut tokens, &mut out);
    }
}
