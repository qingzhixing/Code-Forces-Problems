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

fn validate(s: &Vec<char>) -> bool {
    let mut balance = 0i64;
    for &ch in s {
        if ch == '(' {
            balance += 1;
        } else {
            balance -= 1;
            if balance < 0 {
                return false;
            }
        }
    }
    balance == 0
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let mut a = next_str(tokens);
    let mut b = next_str(tokens);
    let mut assign_to_a = true;
    // 交替分配左括号
    for i in 0..n {
        if a[i] == b[i] {
            continue;
        }
        if assign_to_a {
            a[i] = '(';
            b[i] = ')';
        } else {
            b[i] = '(';
            a[i] = ')';
        }
        assign_to_a = !assign_to_a;
    }

    if validate(&a) && validate(&b) {
        println!("YES");
    } else {
        println!("NO");
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
