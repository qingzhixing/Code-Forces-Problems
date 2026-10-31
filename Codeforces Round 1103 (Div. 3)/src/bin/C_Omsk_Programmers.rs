use std::io::{self, Read};
use std::println;
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
    let mut a: i64 = next(tokens);
    let mut b: i64 = next(tokens);
    let x: i64 = next(tokens);

    let mut result = (a - b).abs();
    let mut div_time = 0;
    while a != b {
        div_time += 1;
        if a > b {
            a /= x;
        } else {
            b /= x;
        }
        result = result.min(div_time + (a - b).abs());
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
