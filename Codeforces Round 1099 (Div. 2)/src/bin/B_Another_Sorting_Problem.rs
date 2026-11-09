use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};

#[allow(dead_code)]
fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str(tokens: &mut SplitAsciiWhitespace) -> Vec<char> {
    tokens.next().unwrap().chars().collect()
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let mut a: Vec<i64> = (0..n).map(|_| next(tokens)).collect();

    if n == 1 {
        println!("YES");
        return;
    }

    let diff: Vec<_> = a.windows(2).map(|w| w[0] - w[1]).collect();
    let max_diff = *diff.iter().max().unwrap();

    // 数组已满足要求
    if max_diff <= 0 {
        println!("YES");
        return;
    }

    for i in 1..n {
        if a[i] < a[i - 1] {
            a[i] += max_diff;
        }
        // 无法弥补
        if a[i] < a[i - 1] {
            println!("NO");
            return;
        }
    }
    println!("YES");
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
