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

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let b: Vec<i64> = (0..n).map(|_| next(tokens)).collect();
    let mut p = vec![None::<usize>; n];
    for x in 1..=n {
        // 0 - indexed
        let pos = b[x - 1] - x as i64 + 1 + n as i64 - 1;
        if pos < 0 || pos > n as i64 - 1 {
            println!("-1");
            return;
        }
        let pos = pos as usize;
        if p[pos] == None {
            p[pos] = Some(x);
        } else {
            println!("-1");
            return;
        }
    }
    for x in p {
        let x = x.unwrap();
        print!("{x} ");
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
