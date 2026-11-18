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
    let mut a: Vec<_> = vec![0usize];

    // 1 - indexed 方便 dp
    a.extend((0..2 * n).map(|_| next::<usize>(tokens)));

    // 处理到当前点的最大得分
    let mut dp = vec![0usize; 2 * n + 1];
    // 当前数字前一个出现的位置
    let mut prev = vec![None::<usize>; n + 1];
    for i in 1..=2 * n {
        // 单独为 1 点匹配
        dp[i] = dp[i - 1] + 1;
        if let Some(prev_pos) = prev[a[i]] {
            let size: usize = i - prev_pos + 1;
            dp[i] = dp[i].max(dp[prev_pos - 1] + size * size);
        } else {
            prev[a[i]] = Some(i);
        }
    }

    println!("{}", dp[2 * n]);
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
