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
    let n = n * 2;
    let _k: usize = next(tokens);
    let s = next_str(tokens);
    let mut odd_out = 0usize;
    let mut even_out = 0usize;
    for (i, &ch) in s.iter().enumerate() {
        if ch == '0' {
            continue;
        }
        // 给不了后面的
        if s[(i + 1) % n] == '1' {
            if i % 2 == 0 {
                even_out += 1;
            } else {
                odd_out += 1;
            }
        } else {
            if i % 2 == 0 {
                odd_out += 1;
            } else {
                even_out += 1;
            }
        }
    }

    println!("{odd_out} {even_out}");
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
