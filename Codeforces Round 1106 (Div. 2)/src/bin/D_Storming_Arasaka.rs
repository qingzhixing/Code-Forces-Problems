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

fn factor_freqs(mut x: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::<(usize, usize)>::new();
    let mut factor = 2usize;
    while factor <= x / factor {
        let mut freq = 0usize;
        if x % factor == 0 {
            while x % factor == 0 {
                x /= factor;
                freq += 1;
            }
            result.push((factor, freq));
        }
        factor += 1;
    }
    if x > 1 {
        result.push((x, 1));
    }
    result
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let factor_freqs = factor_freqs(n);

    // k = 质因子个数 + 频率之和 - 1
    let mut result = 0usize;
    for (_, freq) in factor_freqs {
        result += 1 + freq;
    }
    result -= 1;

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
