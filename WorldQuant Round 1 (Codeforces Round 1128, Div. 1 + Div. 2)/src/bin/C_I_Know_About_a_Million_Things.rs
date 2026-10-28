use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};

const _MAX_N: usize = 0;
const _MAX_M: usize = 0;

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str(tokens: &mut SplitAsciiWhitespace) -> Vec<char> {
    tokens.next().unwrap().chars().collect()
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();
    let mut odd = 0usize;
    let mut even = 0usize;
    for i in 0..n {
        if a[i] % 2 == 0 {
            even += 1;
        } else {
            odd += 1;
        }
    }

    let lose = ((even == 1) && (odd == 1)) || ((even == 0) && (odd == 3));

    println!("{}", if lose { "Larpity" } else { "Moggity" });
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
