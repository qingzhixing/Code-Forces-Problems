use std::io::{self, Read};
use std::println;
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
    let x: usize = next(tokens);
    let y: usize = next(tokens);
    let max_xor = x + y;
    let mut max_x = 0usize;
    for i in (0..31).rev() {
        if (max_xor >> i) & 1 == 1 {
            if max_x + (1 << i) <= x {
                max_x += 1 << i;
            }
        }
    }
    println!("{} {}", max_xor, x - max_x);
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
