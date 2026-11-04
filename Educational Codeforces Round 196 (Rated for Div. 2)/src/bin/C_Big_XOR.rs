use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};

compile_error!("TODO: Wrong Answer.");

#[allow(dead_code)]
fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str(tokens: &mut SplitAsciiWhitespace) -> Vec<char> {
    tokens.next().unwrap().chars().collect()
}

fn msb(num: i64) -> i64 {
    let mut result = 1i64;
    let mut num = num;
    while num > 0 {
        num /= 2;
        result *= 2;
    }
    result / 2
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    for _ in 0..n - 1 {
        print!("0 ");
    }
    println!("1");
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
