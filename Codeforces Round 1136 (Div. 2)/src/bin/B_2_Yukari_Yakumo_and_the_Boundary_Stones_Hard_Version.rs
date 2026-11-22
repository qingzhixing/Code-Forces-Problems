use std::collections::VecDeque;
use std::io::{self, Write};
use std::str::FromStr;

#[allow(dead_code)]
fn next<T: FromStr>(input: &mut VecDeque<String>) -> T {
    while input.is_empty() {
        let mut line = String::new();
        if io::stdin().read_line(&mut line).unwrap() == 0 {
            std::process::exit(0);
        }
        for token in line.split_ascii_whitespace() {
            input.push_back(token.to_string());
        }
    }
    input.pop_front().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str(input: &mut VecDeque<String>) -> Vec<char> {
    next::<String>(input).chars().collect()
}

fn solution(input: &mut VecDeque<String>) {
    let n: i64 = next(input);
    if n == -1 {
        std::process::exit(0);
    }

    let mut s = String::new();
    s.push('+');
    for _ in 0..(n - 1) {
        s.push_str("--+");
    }

    println!("{s}");
    io::stdout().flush().unwrap();

    let k: i64 = next(input);
    if k == -1 {
        std::process::exit(0);
    }

    let y = if k == 0 { 0 } else { (k + 2) / 3 };
    println!("{}", n + y);
    io::stdout().flush().unwrap();
}

fn main() {
    let mut input = VecDeque::new();

    let t: usize = next(&mut input);
    for _ in 0..t {
        solution(&mut input);
    }
}
