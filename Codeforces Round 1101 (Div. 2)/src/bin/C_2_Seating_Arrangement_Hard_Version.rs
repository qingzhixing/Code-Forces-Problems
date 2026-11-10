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
    let str = next_str(tokens);
    let a_cnt = str.iter().filter(|&&ch| ch == 'A').count();

    // 当前 k 个 A 当队头的时候
    let simulate = |k: usize| -> usize {
        // 队列维护空桌
        let mut empty_desk_cnt = x;
        let mut available_seats = 0usize;
        let mut seated = 0usize;
        let mut a_cnt = 0usize;
        for &ch in &str {
            let ch = if ch == 'A' {
                a_cnt += 1;
                // 前 k 个 A 视为 I，后面的视为 E
                if a_cnt <= k { 'I' } else { 'E' }
            } else {
                ch
            };

            match ch {
                'I' => {
                    // 有新桌就开辟
                    if empty_desk_cnt > 0 {
                        empty_desk_cnt -= 1;
                        available_seats += s - 1;
                        seated += 1;
                    }
                    // 否则跳过
                }
                'E' => {
                    // 放在非空桌子，否则跳过
                    if available_seats > 0 {
                        seated += 1;
                        available_seats -= 1;
                    }
                }
                p => {
                    panic!("Invalid Peronality: {p}.");
                }
            }
        }
        seated
    };

    // 二分搜索最合适的 k
    let mut l = 0usize;
    let mut r = a_cnt;
    while l < r {
        let mid = (l + r) >> 1;
        let f_mid = simulate(mid);
        let f_mid_plus_1 = simulate(mid + 1);
        if f_mid < f_mid_plus_1 {
            // 未到最优值
            l = mid + 1;
        } else {
            // 已经开始下坡，或者平坡
            r = mid;
        }
    }

    println!("{}", simulate(l));
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
