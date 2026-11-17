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
    let x: usize = next(tokens);
    let s: usize = next(tokens);
    let kids = next_str(tokens);

    let cnt_a = kids.iter().filter(|&&ch| ch == 'A').count();

    // 计算 前 k 个 A 当做 I, 后 cnt_a - k 个 A 当做 E 的情况下的容纳量
    let get_capacity = |k: usize| -> usize {
        let mut result = 0usize;
        let mut available_table = x;
        let mut available_seat = 0usize;
        let mut cnt_a = 0usize;
        for &kid in &kids {
            let kid = if kid == 'A' {
                cnt_a += 1;
                if cnt_a <= k { 'I' } else { 'E' }
            } else {
                kid
            };
            if kid == 'I' {
                if available_table > 0 {
                    available_table -= 1;
                    available_seat += s - 1;
                    result += 1;
                }
            } else {
                if available_seat > 0 {
                    available_seat -= 1;
                    result += 1;
                }
            }
        }
        result
    };

    let mut l = 0usize;
    let mut r = cnt_a;
    while l < r {
        let mid = (l + r) >> 1;
        if get_capacity(mid) < get_capacity(mid + 1) {
            l = mid + 1;
        } else {
            r = mid;
        }
    }

    println!("{}", get_capacity(l));
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
