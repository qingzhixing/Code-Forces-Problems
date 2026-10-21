use std::io::{self, Read, Write};
use std::str::{FromStr, SplitAsciiWhitespace};

const _MAX_N: usize = 0;
const _MAX_M: usize = 0;

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str<'a>(tokens: &mut SplitAsciiWhitespace<'a>) -> &'a str {
    tokens.next().unwrap()
}

fn solution(tokens: &mut SplitAsciiWhitespace, out: &mut impl Write) {
    let n: usize = next(tokens);
    let a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();
    let b: Vec<usize> = (0..n).map(|_| next(tokens)).collect();

    let mut x = 0; // a = 1, b = 0
    let mut y = 0; // a = 0, b = 1
    let mut has_one_a = false;
    let mut has_zero_b = false;
    for i in 0..n {
        if a[i] == 1 {
            has_one_a = true;
        }
        if b[i] == 0 {
            has_zero_b = true;
        }
        if a[i] != b[i] {
            if a[i] == 1 {
                x += 1;
            } else {
                y += 1;
            }
        }
    }

    // 完全相等
    if x == 0 && y == 0 {
        writeln!(out, "0").unwrap();
        return;
    }

    if x == 0 {
        if has_one_a && has_zero_b {
            writeln!(out, "2").unwrap();
        } else {
            writeln!(out, "-1").unwrap();
        }
    } else if x % 2 == 1 {
        writeln!(out, "1").unwrap();
    } else {
        writeln!(out, "2").unwrap();
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
    out.flush().unwrap();
}
