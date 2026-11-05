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
    let d = n.ilog2() as usize;
    let mut current_m = (1..=n).fold(0usize, |prev, value| prev ^ value);
    let mut result = vec![false; n + 1];

    // 0 - indexed 从 第 d+1 位往第 1 位枚举翻转
    for idx in (1..=d + 1).rev() {
        if current_m >> idx & 1 == 0 {
            current_m ^= (1 << idx) ^ (1 << (idx - 1));
            result[1 << (idx - 1)] = true;
        }
    }

    for &item in &result[1..] {
        print!("{} ", if item { 1 } else { 0 });
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
