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
    let mut odd_pos: Vec<i64> = vec![];
    let mut odd_neg: Vec<i64> = vec![];
    let mut even_pos: Vec<i64> = vec![];
    let mut even_neg: Vec<i64> = vec![];

    for (idx, num) in a.into_iter().enumerate() {
        // 1 - indexed
        let idx = idx + 1;
        if num <= 0 {
            if idx % 2 == 0 {
                even_neg.push(num);
            } else {
                odd_neg.push(num);
            }
        } else {
            if idx % 2 == 0 {
                even_pos.push(num);
            } else {
                odd_pos.push(num);
            }
        }
    }

    even_neg.sort_unstable();
    odd_neg.sort_unstable();
    even_pos.sort_unstable();
    odd_pos.sort_unstable();

    let mut odd_tagged = false;
    let mut even_tagged = false;
    for _ in 0..m {
        let x: usize = next(tokens);
        if x % 2 == 0 {
            if !even_pos.is_empty() {
                even_tagged = true;
                even_pos.pop();
            } else if !even_tagged {
                even_tagged = true;
                even_neg.pop();
            }
        } else {
            if !odd_pos.is_empty() {
                odd_tagged = true;
                odd_pos.pop();
            } else if !odd_tagged {
                odd_tagged = true;
                odd_neg.pop();
            }
        }
    }
    let result = even_pos.into_iter().sum::<i64>()
        + odd_pos.into_iter().sum::<i64>()
        + even_neg.into_iter().sum::<i64>()
        + odd_neg.into_iter().sum::<i64>();
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
