use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};
use std::{iter, println};

const _MAX_N: usize = 0;
const _MAX_M: usize = 0;

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str<'a>(tokens: &mut SplitAsciiWhitespace<'a>) -> &'a str {
    tokens.next().unwrap()
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let m: usize = next(tokens);
    let a: Vec<i64> = iter::once(0).chain((0..n).map(|_| next(tokens))).collect();
    let mut b: Vec<usize> = iter::once(0).chain((0..m).map(|_| next(tokens))).collect();
    let pre_sum: Vec<i64> = a
        .into_iter()
        .scan(0i64, |prev, x| {
            *prev += x;
            Some(*prev)
        })
        .collect();

    b.sort_unstable();
    let b = b;

    let mut result = 0i64;
    for idx in 1..=m {
        result += (pre_sum[b[idx]] - pre_sum[b[idx - 1]]).abs();
    }
    result += pre_sum[n] - pre_sum[b[m]];

    println!("{result}");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    let t: usize = next(&mut tokens);
    for _ in 0..t {
        solution(&mut tokens);
    }
}
