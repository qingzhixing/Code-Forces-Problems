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
    let a = next_str(tokens);
    let b = next_str(tokens);

    // 处理完前 i 列的最小代价
    let mut dp = vec![1e9 as usize; n + 2];
    dp[0] = 0;
    for i in 0..n {
        let cost_c = (a[i] != b[i]) as usize;
        dp[i + 1] = dp[i + 1].min(dp[i] + cost_c);
        if i + 1 < n {
            let cost_v = (a[i] != a[i + 1]) as usize + (b[i] != b[i + 1]) as usize;
            dp[i + 2] = dp[i + 2].min(dp[i] + cost_v);
        }
    }

    println!("{}", dp[n]);
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
