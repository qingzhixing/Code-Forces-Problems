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
    let mut l: Vec<_> = vec![0i64; n];
    let mut r: Vec<_> = vec![0i64; n];
    let mut u: Vec<_> = vec![0i64; n];
    let mut v: Vec<_> = vec![0i64; n];
    for i in 0..n {
        l[i] = next(tokens);
        r[i] = next(tokens);
        u[i] = next(tokens);
        v[i] = next(tokens);
    }

    // 枚举最大长度
    for m in (1..=n as i64).rev() {
        let mut j = 1i64;
        // 从左到右枚举每一个点，尝试放入
        for i in 0..n {
            let left_ok = j < l[i] || j > r[i];
            let right_ok = (m - j + 1) < u[i] || (m - j + 1) > v[i];
            if left_ok && right_ok {
                j += 1;
            }
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
