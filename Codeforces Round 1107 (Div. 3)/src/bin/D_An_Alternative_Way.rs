use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};

const _MAX_N: usize = 0;
const _MAX_M: usize = 0;

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str(tokens: &mut SplitAsciiWhitespace) -> Vec<char> {
    tokens.next().unwrap().chars().collect()
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let a: Vec<i64> = (0..n).map(|_| next(tokens)).collect();
    let b: Vec<i64> = (0..n).map(|_| next(tokens)).collect();
    let delta: Vec<i64> = (0..n).map(|idx| b[idx] - a[idx]).collect();

    // delta 每个前缀和一定 >= 0 才行
    let pre_sum: Vec<i64> = delta
        .into_iter()
        .scan(0i64, |prev, value| {
            *prev += value;
            Some(*prev)
        })
        .collect();

    let bad = pre_sum.iter().any(|&value| value < 0);
    println!("{}", if bad { "NO" } else { "YES" });
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
