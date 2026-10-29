use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};

const _MAX_N: usize = 60;
const _MAX_M: usize = 50;

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
    let mut result = a[0];

    // 当前最小和最大的可能前缀和
    let mut min_s = a[0];
    let mut max_s = a[0];

    for i in 1..n {
        result = result.max(a[i] + max_s);
        result = result.max(a[i] - min_s);

        min_s = a[i].min(a[i] + 2 * min_s);
        max_s = a[i].max(a[i] + 2 * max_s);
    }

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
