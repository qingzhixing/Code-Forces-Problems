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
    let _n: usize = next(tokens);
    let k: usize = next(tokens);
    let s: Vec<usize> = next_str(tokens)
        .into_iter()
        .map(|x| x as usize - '0' as usize)
        .collect();

    let mut cnt1 = s.iter().filter(|&&x| x == 1).count();
    let mut cnt0 = s.iter().filter(|&&x| x == 0).count();

    if cnt1 > cnt0 {
        if cnt0 <= k {
            cnt1 += cnt0;
            cnt0 = 0;
        } else {
            cnt0 -= k;
            cnt1 += k;
        }
    } else {
        if cnt1 <= k {
            cnt0 += cnt1;
            cnt1 = 0;
        } else {
            cnt1 -= k;
            cnt0 += k;
        }
    }

    println!("{}", cnt0 * (cnt0 + 1) / 2 + cnt1 * (cnt1 + 1) / 2 + 1);
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
