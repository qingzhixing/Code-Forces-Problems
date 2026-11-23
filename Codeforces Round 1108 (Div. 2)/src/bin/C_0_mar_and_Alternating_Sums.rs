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

const MOD: usize = 1e9 as usize + 7;

fn quick_power(mut base: usize, mut power: usize) -> usize {
    base %= MOD;
    let mut result = 1usize;
    while power > 0 {
        if power & 1 == 1 {
            result *= base;
            result %= MOD;
        }

        power >>= 1;
        base *= base;
        base %= MOD;
    }

    result
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let mut a: Vec<i64> = (0..n).map(|_| next(tokens)).collect();

    let has_neg1 = a[0] == -1;

    // 有序 a 去重
    a.dedup();
    let d = a.len();

    if !has_neg1 {
        println!("{}", quick_power(2, n - d));
        return;
    }

    let l = a.windows(2).filter(|w| w[0] + 1 == w[1]).count();
    println!("{}", quick_power(2, n - d) * (l + 1) % MOD);
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
