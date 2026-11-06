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

/// 返回字符串中未匹配，
fn valid(str: &Vec<char>) -> bool {
    let mut balance = 0i64;
    for &ch in str {
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
    let mut s1 = next_str(tokens);
    let mut s2 = next_str(tokens);

    let total_open = s1.iter().chain(s2.iter()).filter(|&&ch| ch == '(').count();

    // 序列中总 '(' 一定等于总 ')' 数
    if total_open != n {
        println!("NO");
        return;
    }

    // 交替分配左括号
    let mut open_s1 = true;
    for i in 0..n {
        if s1[i] != s2[i] {
            if open_s1 {
                s1[i] = '(';
                s2[i] = ')';
            } else {
                s1[i] = ')';
                s2[i] = '(';
            }
            open_s1 = !open_s1;
        }
    }

    if valid(&s1) && valid(&s2) {
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
