use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};
use std::{println, vec};

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
    let l: usize = next(tokens);
    let a: usize = next(tokens);
    let b: usize = next(tokens);
    let lcd = a * b / gcd(a, b);
    let mut visited = vec![false; l];
    let mut nxt = 0usize;
    let mut result = 0usize;
    while !visited[nxt] {
        visited[nxt] = true;
        result += 1;
        nxt = (nxt + lcd) % l;
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
