use std::cmp::{max, min};
use std::io::{self, Read, Write};
use std::str::{FromStr, SplitAsciiWhitespace};

const _MAX_N: usize = 1e3 as usize + 10;
const _MAX_M: usize = 10;

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

fn solution(tokens: &mut SplitAsciiWhitespace, out: &mut impl Write) {
    let a: i32 = next(tokens);
    let b: i32 = next(tokens);
    let c: i32 = next(tokens);

    let maximum = max(a, max(b, c));
    let minimum = min(a, min(b, c));
    let middle = a + b + c - maximum - minimum;

    writeln!(out, "{}", min(maximum - middle, middle - minimum)).unwrap();
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
