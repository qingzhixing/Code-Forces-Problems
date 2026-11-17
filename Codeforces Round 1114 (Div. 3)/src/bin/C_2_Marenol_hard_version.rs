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
    let a = next_str(tokens);
    let b = next_str(tokens);
    let mut a_odd_1 = 0usize;
    let mut a_even_1 = 0usize;
    for (i, &ch) in a.iter().enumerate() {
        if ch == '1' {
            if i % 2 == 0 {
                a_even_1 += 1;
            } else {
                a_odd_1 += 1;
            }
        }
    }
    let mut b_odd_1 = 0usize;
    let mut b_even_1 = 0usize;
    for (i, &ch) in b.iter().enumerate() {
        if ch == '1' {
            if i % 2 == 0 {
                b_even_1 += 1;
            } else {
                b_odd_1 += 1;
            }
        }
    }

    if a_odd_1 != b_odd_1 || a_even_1 != b_even_1 {
        println!("-1");
        return;
    }

    let a_odds: Vec<_> = a
        .iter()
        .enumerate()
        .filter(|&(i, &ch)| i % 2 == 1 && ch == '1')
        .map(|(i, _)| i)
        .collect();
    let a_evens: Vec<_> = a
        .iter()
        .enumerate()
        .filter(|&(i, &ch)| i % 2 == 0 && ch == '1')
        .map(|(i, _)| i)
        .collect();
    let b_odds: Vec<_> = b
        .iter()
        .enumerate()
        .filter(|&(i, &ch)| i % 2 == 1 && ch == '1')
        .map(|(i, _)| i)
        .collect();
    let b_evens: Vec<_> = b
        .iter()
        .enumerate()
        .filter(|&(i, &ch)| i % 2 == 0 && ch == '1')
        .map(|(i, _)| i)
        .collect();

    let mut result = 0usize;
    for i in 0..a_odds.len() {
        result += ((a_odds[i] as i64 - b_odds[i] as i64).abs() as usize) / 2;
    }
    for i in 0..a_evens.len() {
        result += ((a_evens[i] as i64 - b_evens[i] as i64).abs() as usize) / 2;
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
