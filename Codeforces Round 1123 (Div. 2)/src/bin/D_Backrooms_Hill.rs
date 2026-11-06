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
    let a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();
    // 当前数字对应下标的奇偶性 1 奇 0 偶
    let mut idx_parity = vec![false; n + 1];
    for (idx, val) in a.into_iter().enumerate() {
        idx_parity[val] = idx % 2 == 1;
    }

    let mut balance = 0i64;
    for val in (1..=n).rev() {
        balance += if idx_parity[val] { 1 } else { -1 };
        if balance.abs() > 1 {
            println!("NO");
            return;
        }
    }
    println!("YES");
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
