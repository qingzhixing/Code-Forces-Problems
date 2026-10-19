use std::io::{self, Read, Write};
use std::str::{FromStr, SplitAsciiWhitespace};

const _MAX_N: usize = 2e5 as usize + 10;

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

fn solution(tokens: &mut SplitAsciiWhitespace, out: &mut impl Write) {
    let n: usize = next(tokens);
    let a: Vec<char> = tokens.next().unwrap().chars().collect();
    let b: Vec<char> = tokens.next().unwrap().chars().collect();

    let mut result = 0i64;

    // 奇偶单独考虑
    for start in 0..2 {
        let mut a_pos: Vec<i64> = vec![];
        let mut b_pos: Vec<i64> = vec![];

        for idx in (start..n).step_by(2) {
            if a[idx] == '1' {
                a_pos.push(idx as i64);
            }
            if b[idx] == '1' {
                b_pos.push(idx as i64);
            }
        }

        if a_pos.len() != b_pos.len() {
            writeln!(out, "-1").unwrap();
            return;
        }

        for idx in 0..a_pos.len() {
            // 一次能走 2 格
            result += (a_pos[idx] - b_pos[idx]).abs() / 2;
        }
    }

    writeln!(out, "{result}").unwrap();
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
