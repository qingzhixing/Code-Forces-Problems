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

    let mut equal = vec![0i64; n + 1];
    let mut diff = vec![0i64; n + 1];

    for i in 0..n {
        // 00 11
        equal[i + 1] = equal[i] + ((s[i] == t[i]) as i64);
        // 10 -> +1; 01 -> -1;
        diff[i + 1] = diff[i] + s[i] as i64 - t[i] as i64;
    }

    for _ in 0..q {
        let l: usize = next(tokens);
        let r: usize = next(tokens);
        let range_equal = equal[r] - equal[l - 1];
        let range_diff = diff[r] - diff[l - 1];
        if range_equal >= range_diff.abs() {
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
