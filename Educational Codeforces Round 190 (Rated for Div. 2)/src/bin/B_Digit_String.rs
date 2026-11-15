use std::io::{self, Read};
use std::println;
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
    let n = s.len();

    // 答案上界
    let mut result = n;

    let cnt4 = s.iter().filter(|&&ch| ch == '4').count();
    let cnt2 = s.iter().filter(|&&ch| ch == '2').count();

    // 删除 4
    let s: Vec<char> = s.into_iter().filter(|&ch| ch != '4').collect();

    // 从左到右枚举删除线，删除线左侧的 1/3 全被删除，右侧的 2 全被删除
    // 前缀 2 统计
    let mut pre2 = 0usize;
    for (idx, ch) in s.into_iter().enumerate() {
        let ch = ch as usize - '0' as usize;
        let deleted_13 = idx - pre2;
        let deleted_2 = cnt2 - pre2;
        result = result.min(cnt4 + deleted_13 + deleted_2);
        if ch == 2 {
            pre2 += 1;
        }
    }

    // 全 2
    result = result.min(n - cnt2);

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
