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
    let k: usize = next(tokens);
    let segment = n - k;

    if segment <= 1 {
        println!("-1");
        return;
    }

    let chars = ["0", "1"];
    let prefix: String = chars.iter().copied().cycle().take(segment - 2).collect();
    let second_last = chars[(segment - 2) & 1].repeat(1 + k - k / 2);
    let last = chars[(segment - 1) & 1].repeat(1 + k / 2);

    println!("{prefix}{second_last}{last}");
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
