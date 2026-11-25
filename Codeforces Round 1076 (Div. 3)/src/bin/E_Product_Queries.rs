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

const INF: usize = 1e9 as usize;
fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let mut a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();
    a.sort_unstable();
    a.dedup();

    let mut dp = vec![INF; n + 1];
    for x in a {
        dp[x] = 1;
        if x == 1 {
            continue;
        }
		for prev in 2..=n / x {
            if dp[prev] != INF {
                let nxt = prev * x;
                dp[nxt] = dp[nxt].min(dp[prev] + 1);
            }
        }
    }

    for i in 1..=n {
        if dp[i] == INF {
            print!("-1 ");
        } else {
            print!("{} ", dp[i]);
        }
    }
    println!("");
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
