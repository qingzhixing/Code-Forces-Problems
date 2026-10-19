use std::cmp::max;
use std::io::{self, Read, Write};
use std::str::{FromStr, SplitAsciiWhitespace};

const _MAX_N: usize = 2e5 as usize + 10;
const _MAX_M: usize = _MAX_N;

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

fn solution(tokens: &mut SplitAsciiWhitespace, out: &mut impl Write) {
    let n: usize = next(tokens);
    let s: Vec<char> = tokens.next().unwrap().chars().collect();

    let mut last_ch = '\0';
    let mut result = 0i32;
    s.iter().for_each(|ch| {
        if last_ch != *ch {
            result += 1;
        }
        last_ch = *ch;
    });

    // 删除后最多减少的长度
    let mut delta = 0i32;
    for id in 1..n - 1 {
        if s[id - 1] != s[id] && s[id] != s[id + 1] {
            delta = max(1, delta);
            if s[id - 1] == s[id + 1] {
                delta = max(2, delta);
            }
        }
    }

    writeln!(out, "{}", result - delta).unwrap();
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
