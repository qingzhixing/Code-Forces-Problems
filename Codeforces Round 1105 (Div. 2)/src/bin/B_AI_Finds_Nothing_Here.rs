use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};

const _MAX_N: usize = 1e9 as usize;
const _MAX_M: usize = 1e9 as usize;
const MOD: usize = 998244353;

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str(tokens: &mut SplitAsciiWhitespace) -> Vec<char> {
    tokens.next().unwrap().chars().collect()
}

fn quick_power(mut base: usize, mut power: usize, modular: usize) -> usize {
    let mut result = 1usize;
    while power > 0 {
        if power & 1 == 1 {
            result = result * base % modular;
        }
        base = base * base % modular;
        power >>= 1;
    }
    result
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let m: usize = next(tokens);
    let r: usize = next(tokens);
    let c: usize = next(tokens);
    let power = n * m - (n - r + 1) * (m - c + 1);
    let result = quick_power(2, power, MOD);
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
