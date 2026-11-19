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
    let b: Vec<usize> = (0..n).map(|_| next(tokens)).collect();

    // 1 -> 0
    let mut need_reset = 0usize;
    // 0 -> 1
    let mut need_set = 0usize;
    let mut double_zero = false;
    let mut double_one = false;

    for i in 0..n {
        match (a[i], b[i]) {
            (0, 0) => {
                double_zero = true;
            }
            (1, 0) => {
                need_reset += 1;
            }
            (0, 1) => {
                need_set += 1;
            }
            (1, 1) => {
                double_one = true;
            }
            (from, to) => {
                panic!("Invalid Input: {} -> {}", from, to);
            }
        }
    }

    if need_reset == 0 && need_set == 0 {
        println!("0");
        return;
    }

    if need_reset != 0 {
        // 一定有解
        println!("{}", [2, 1][need_reset & 1]);
        return;
    }

    // 此时只有 0 -> 1
    // 唯一有解情况是借助 0 -> 0 1 -> 1 生成一个 1 -> 0
    if !double_one || !double_zero {
        println!("-1");
        return;
    }

    println!("2");
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
