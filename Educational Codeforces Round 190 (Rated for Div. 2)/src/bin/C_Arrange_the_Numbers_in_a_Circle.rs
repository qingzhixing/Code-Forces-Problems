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
    let mut result = 0usize;
    // 能容纳单独一张牌的数量
    let mut capacity = 0usize;
    let mut not_1_cnt = 0usize;
    for num in a.into_iter().rev() {
        if num > 1 {
            result += num;
            not_1_cnt += 1;
            capacity += (num / 2) - 1;
        } else {
            if not_1_cnt == 1 {
                capacity += 1;
                not_1_cnt = 0;
            }
            if capacity > 0 {
                capacity -= 1;
                result += 1;
            }
        }
    }

    if result <= 2 {
        println!("0");
    } else {
        println!("{result}");
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
