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
    let q: usize = next(tokens);
    let s = next_str(tokens);
    let t = next_str(tokens);

    let mut same = vec![0i64; n + 1];
    let mut diff = vec![0i64; n + 1];
    for i in 0..n {
        same[i + 1] = same[i] + (s[i] == t[i]) as i64;
        // 10 -> +1, 01 -> -1, 11/00 -> 0
        diff[i + 1] = diff[i] + (s[i] as i64 - t[i] as i64);
    }

    for _ in 0..q {
        let l: usize = next(tokens);
        let r: usize = next(tokens);

        let same = same[r] - same[l - 1];
        let diff = diff[r] - diff[l - 1];

        if diff.abs() <= same {
            println!("YES");
        } else {
            println!("NO");
        }
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
