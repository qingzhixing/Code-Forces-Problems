use std::cmp::Reverse;
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
    let c: i64 = next(tokens);
    let mut a: Vec<i64> = (0..n)
        .map(|_| next(tokens))
        .map(|number: i64| number - c)
        .collect();

    // 从大到小排序
    a.sort_unstable_by_key(|&x| Reverse(x));
    let a = a;

    let positive_cnt = a.partition_point(|&x| x > 0);
    let operation_cnt = positive_cnt.max(n.div_ceil(2));

    let result: i64 = a.into_iter().take(operation_cnt).sum();
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
    out.flush().unwrap();
}
