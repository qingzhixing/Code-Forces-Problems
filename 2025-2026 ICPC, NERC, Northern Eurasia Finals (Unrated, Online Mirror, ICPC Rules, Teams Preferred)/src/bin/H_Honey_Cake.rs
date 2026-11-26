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

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let w: usize = next(tokens);
    let h: usize = next(tokens);
    let d: usize = next(tokens);
    let mut n: usize = next(tokens);

    let w_pieces = gcd(n, w);
    n /= w_pieces;
    let h_pieces = gcd(n, h);
    n /= h_pieces;
    let d_pieces = gcd(n, d);
    n /= d_pieces;

    if n != 1 {
        println!("-1");
        return;
    }

    println!("{} {} {}", w_pieces - 1, h_pieces - 1, d_pieces - 1);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    solution(&mut tokens);
}
