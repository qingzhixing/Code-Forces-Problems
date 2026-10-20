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
    let w: Vec<i64> = (0..n).map(|_| next(tokens)).collect();

    if n % 2 == 1 {
        writeln!(out, "NO").unwrap();
        return;
    }

    let mut l = 0i64;
    let mut r = i64::MAX;
    for pair_idx in 0..(n / 2) {
        let start = w[pair_idx * 2 + 1];
        let end = w[pair_idx * 2];

        // 当前区间不合法 或者 与 [l, r] 无交集
        if start > end || end < l || start > r {
            writeln!(out, "NO").unwrap();
            return;
        }

        l = l.max(start);
        r = r.min(end);
    }

    // l, r 长度 >= 3
    if r - l - 2 >= 0 {
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
    out.flush().unwrap();
}
