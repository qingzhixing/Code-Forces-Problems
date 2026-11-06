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
    let d = n.ilog2();
    let mut m = (1..=n).fold(0usize, |prev, value| prev ^ value);
    let mut doubled = vec![false; n + 1];
    for digit in (1..=d + 1).rev() {
        if m >> digit & 1 == 0 {
            doubled[1 << (digit - 1)] = true;
            m ^= (1 << (digit - 1)) ^ (1 << digit);
        }
    }

    for num in 1..=n {
        print!("{} ", if doubled[num] { 1 } else { 0 });
    }
    println!("");
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
