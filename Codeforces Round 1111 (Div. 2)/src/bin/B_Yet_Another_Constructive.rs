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
    let k: usize = next(tokens);
    let m: usize = next(tokens);

    if k > m {
        writeln!(out, "NO").unwrap();
        return;
    } else {
        writeln!(out, "YES").unwrap();
    }

    if k == 1 {
        for _ in 0..n {
            write!(out, "{m} ").unwrap();
        }
        writeln!(out).unwrap();
        return;
    }

    let s_remainder: Vec<usize> = (0..n)
        .into_iter()
        .enumerate()
        .map(|(idx, _)| k - idx % k - 1)
        .collect();

    // 构造原数组前缀和
    let s: Vec<usize> = s_remainder
        .into_iter()
        .enumerate()
        .map(|(idx, remainder)| remainder + idx * m)
        .collect();

    // 还原原数组
    let a: Vec<usize> = s
        .into_iter()
        .scan(0usize, |previous, current| {
            let diff = current - *previous;
            *previous = current;
            Some(diff)
        })
        .collect();

    // output
    a.into_iter()
        .for_each(|number| write!(out, "{number} ").unwrap());
    writeln!(out).unwrap();
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
