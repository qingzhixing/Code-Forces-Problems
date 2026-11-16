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

const MOD: usize = 998244353usize;

fn quick_power(mut base: usize, mut power: usize) -> usize {
    let mut result = 1usize;
    base %= MOD;
    while power > 0 {
        if power & 1 == 1 {
            result = result * base % MOD;
        }
        power >>= 1;
        base = base * base % MOD;
    }
    result
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let mut a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();
    a.sort_unstable();
    a.iter_mut().for_each(|x| *x %= MOD);

    // (n-1)!
    let factorial = (1..n).fold(1usize, |prev, val| prev * val % MOD);
    // suf_sum[i] = a[i+1]+..+a[n-1]
    let mut suf_sum = vec![0; n];
    for i in (0..n - 1).rev() {
        suf_sum[i] = (suf_sum[i + 1] + a[i + 1]) % MOD;
    }

    let mut result = 0usize;
    // 枚举除去 root 的节点
    for i in 0..n - 1 {
        // 计算 (n-1)!/(n-1-i) 使用模逆元求解
        let choice = factorial * quick_power(n - 1 - i, MOD - 2) % MOD;
        result += choice * (suf_sum[i] + MOD - a[i] * (n - 1 - i) % MOD) % MOD;
        result %= MOD;
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
