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
    let mut a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();
    a.sort_unstable();

    let mut max_consistent = 0usize;

    for i in 0..n {
        // 与当前数字相容的数字个数
        let mut consistent = 0usize;
        for j in i..n {
            if a[i] % a[j] == 0 || a[j] % a[i] == 0 {
                consistent += 1;
            }
        }

        max_consistent = max_consistent.max(consistent);
    }

    println!("{}", n - max_consistent);
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
