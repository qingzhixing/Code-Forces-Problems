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
    let mut l: Vec<i64> = vec![0i64; n];
    let mut r: Vec<i64> = vec![0i64; n];
    let mut u: Vec<i64> = vec![0i64; n];
    let mut v: Vec<i64> = vec![0i64; n];

    for i in 0..n {
        l[i] = next(tokens);
        r[i] = next(tokens);
        u[i] = next(tokens);
        v[i] = next(tokens);
    }

    for m in (1..=n as i64).rev() {
        // 当前正在放置的位置
        let mut j = 1i64;
        for i in 0..n {
            let left_rank = j;
            let right_rank = m - j + 1;
            if left_rank >= l[i] && left_rank <= r[i] {
                continue;
            }
            if right_rank >= u[i] && right_rank <= v[i] {
                continue;
            }
            j += 1;
            if j > m {
                break;
            }
        }
        if j > m {
            println!("{m}");
            return;
        }
    }

    println!("0");
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
