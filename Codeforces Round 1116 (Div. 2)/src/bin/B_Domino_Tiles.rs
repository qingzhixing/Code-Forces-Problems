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
    let s = next_str(tokens);
    // 奇数位置确定下来的第一个坐标
    let mut odd_confirmed_idx = None;
    // 偶数位置确定下来的第一个坐标
    let mut even_confirmed_idx = None;
    for i in 0..n {
        let ch = s[i];
        if ch == '?' {
            continue;
        }
        if i % 2 == 0 {
            if even_confirmed_idx == None {
                even_confirmed_idx = Some(i);
                continue;
            }
            if (ch != s[even_confirmed_idx.unwrap()])
                == ((i - even_confirmed_idx.unwrap()) / 2 % 2 == 0)
            {
                println!("0");
                return;
            }
        } else {
            if odd_confirmed_idx == None {
                odd_confirmed_idx = Some(i);
                continue;
            }
            if (ch != s[odd_confirmed_idx.unwrap()])
                == ((i - odd_confirmed_idx.unwrap()) / 2 % 2 == 0)
            {
                println!("0");
                return;
            }
        }
    }

    let result: usize = if odd_confirmed_idx == None { 2 } else { 1 }
        * if even_confirmed_idx == None { 2 } else { 1 };

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
