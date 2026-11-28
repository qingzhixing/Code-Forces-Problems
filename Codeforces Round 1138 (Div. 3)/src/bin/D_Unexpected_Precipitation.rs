use std::io::{self, Read};
use std::println;
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
    let x: Vec<i64> = (0..n).map(|_| next(tokens)).collect();
    let y: Vec<i64> = (0..n).map(|_| next(tokens)).collect();

    // dp[i]: 考虑 i ~ n - 1 对商店时，当前在左侧的人在双方均最优决策情况下
    // 所能得到的最大差值 (左 - 右)
    let mut dp = 0i64;
    for i in (0..n).rev() {
        let delta = x[i] - y[i];
        // Skip, Buy
        dp = (-dp).max(delta + dp);
    }

    if dp > 0 {
        println!("Sunny");
    } else if dp < 0 {
        println!("Aubrey");
    } else {
        println!("Draw");
    }
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
