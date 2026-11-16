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
    a.sort_unstable_by(|x, y| y.cmp(x));
    let a = a;

    // 不 op 将产生一个答案
    let mut result = 1usize;
    for i in 0..n {
        // 重复的数字会被锁死
        if i < n - 1 && a[i] == a[i + 1] {
            break;
        }
        result = (result + quick_power(a[i] - 1, i + 1)) % MOD;
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
