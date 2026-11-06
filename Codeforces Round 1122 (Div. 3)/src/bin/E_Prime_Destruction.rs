use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};
use std::sync::LazyLock;

#[allow(dead_code)]
fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str(tokens: &mut SplitAsciiWhitespace) -> Vec<char> {
    tokens.next().unwrap().chars().collect()
}

const MAX_N: usize = 2e5 as usize + 10;

/// factors[i] 存储 i 的所有不同素因子（升序）
static FACTORS: LazyLock<Vec<Vec<usize>>> = LazyLock::new(|| {
    let mut factors: Vec<Vec<usize>> = vec![Vec::new(); MAX_N];
    for p in 2..MAX_N {
        // factors[p] 仍为空 ⇒ p 是素数
        // 因为任何 < p 的素因子都会在之前的循环中把 p 标记为“非空”
        if factors[p].is_empty() {
            let mut m = p;
            while m < MAX_N {
                factors[m].push(p);
                m += p;
            }
        }
    }
    factors
});

/// 便捷查询：返回 n 的所有不同素因子
pub fn prime_factors(n: usize) -> &'static [usize] {
    &FACTORS[n]
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let k: usize = next(tokens);
    let a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();

    // 当前数字变为 <= k 的最小代价
    let mut dp = vec![1145141919810usize; n + 1];
    for i in 0..=k {
        dp[i] = 0;
    }
    for i in k + 1..=n {
        for &prime in prime_factors(i) {
            dp[i] = dp[i].min(1 + prime * dp[i / prime]);
        }
    }

    let result: usize = a.into_iter().map(|value| dp[value]).sum();
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
