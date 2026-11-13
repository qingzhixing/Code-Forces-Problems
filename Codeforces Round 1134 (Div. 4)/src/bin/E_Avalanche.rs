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
    let a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();
    let mut a_win = 0usize;
    for (idx, num) in a.into_iter().enumerate() {
        if idx % (k + 1) == 0 {
            a_win ^= num & 1;
        }
    }
    println!("{}", if a_win == 1 { "Alice" } else { "Bob" });
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
