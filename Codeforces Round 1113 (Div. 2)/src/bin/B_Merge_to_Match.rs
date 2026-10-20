use std::io::{self, Read, Write};
use std::str::{FromStr, SplitAsciiWhitespace};

const _MAX_N: usize = 2e5 as usize;
const _MAX_M: usize = 1e9 as usize;

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

fn solution(tokens: &mut SplitAsciiWhitespace, out: &mut impl Write) {
    let n: usize = next(tokens);
    let m: usize = next(tokens);
    let mut a: Vec<i64> = (0..n).map(|_| next(tokens)).collect();
    let mut b: Vec<i64> = (0..m).map(|_| next(tokens)).collect();

    if n < 2 * m {
        writeln!(out, "NO").unwrap();
        return;
    }

    a.sort_unstable();
    b.sort_unstable();
    let a = a;
    let b = b;

    for idx in 0..m {
        if b[idx] <= a[idx] || b[idx] >= a[n - m + idx] {
            writeln!(out, "NO").unwrap();
            return;
        }
    }
    writeln!(out, "YES").unwrap();
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
