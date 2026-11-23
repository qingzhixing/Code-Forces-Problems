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
    let m: usize = next(tokens);
    let a: Vec<i64> = (0..n).map(|_| next(tokens)).collect();
    let mut b: Vec<usize> = (0..m).map(|_| next(tokens)).collect();
    b.push(0);
    b.push(n);
    b.sort_unstable();

    let mut result = 0i64;
    for i in 0..b.len() - 1 {
        let sum: i64 = a[b[i]..b[i + 1]].iter().sum();
        if i != b.len() - 2 {
            result += sum.abs();
        } else {
            result += sum;
        }
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
