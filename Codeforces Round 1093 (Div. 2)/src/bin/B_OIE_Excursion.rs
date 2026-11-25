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
    let a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();

    let mut prev = None;
    let mut cnt = 0usize;
    for x in a {
        if Some(x) == prev {
            cnt += 1;
        } else {
            prev = Some(x);
            if cnt >= m {
                println!("NO");
                return;
            }
            cnt = 1;
        }
    }
    if cnt >= m {
        println!("NO");
        return;
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
