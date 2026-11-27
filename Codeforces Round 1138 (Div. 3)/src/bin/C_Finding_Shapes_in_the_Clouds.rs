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
    let n: usize = next(tokens);
    let s = next_str(tokens);
    if n % 2 == 1 {
        println!("NO");
        return;
    }

    if n == 2 {
        if s[0] != s[1] {
            println!("NO");
        } else {
            println!("YES");
        }
        return;
    }

    if s.windows(2).all(|w| w[0] != w[1]) {
        println!("NO");
    } else {
        println!("YES");
    }
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
