use std::io::{self, Read};
use std::println;
use std::str::{FromStr, SplitAsciiWhitespace};

const _MAX_N: usize = 2e5 as usize + 10;
const _MAX_M: usize = 1e9 as usize;
const MOD: i64 = 1e9 as i64 + 7;

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str<'a>(tokens: &mut SplitAsciiWhitespace<'a>) -> &'a str {
    tokens.next().unwrap()
}

fn quick_power(mut base: i64, mut power: i64, modular: i64) -> i64 {
    let mut result = 1i64;
    while power > 0 {
        if power & 1 == 1 {
            result = result * base % modular;
        }
        power >>= 1;
        base = base * base % modular;
    }
    result
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let a: Vec<i64> = (0..n).map(|_| next(tokens)).collect();

    // 不同数字的个数
    let d = 1 + a.windows(2).filter(|w| w[0] != w[1]).count();
    // 数组中是否有 -1
    let has_negative_one = a[0] == -1;
    // 正整数 (v, v+1) 对数
    let l = a
        .windows(2)
        .filter(|w| w[0] > 0 && w[1] == w[0] + 1)
        .count();

    if !has_negative_one {
        println!("{}", quick_power(2, (n - d) as i64, MOD));
    } else {
        println!(
            "{}",
            quick_power(2, (n - d) as i64, MOD) * ((l + 1) as i64) % MOD
        );
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
