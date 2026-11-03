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
    let s = next_str(tokens);
    let len = s.len();
    let total_odds = s
        .iter()
        .filter(|&&ch| (ch as usize - '0' as usize) % 2 == 1)
        .count();
    let mut left_2 = 0usize;
    let mut right_odd = total_odds;

    // 从前往后遍历分段点，更新能保留的最多字符数
    // 分段点前侧仅保留 2，分段点后仅保留奇数
    let mut max_reserved = total_odds;
    for ch in s {
        let num = ch as usize - '0' as usize;
        if num == 4 {
            continue;
        }
        if num == 2 {
            left_2 += 1;
        } else {
            right_odd -= 1;
        }

        max_reserved = max_reserved.max(left_2 + right_odd);
    }

    let result = len - max_reserved;
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
