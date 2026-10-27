use std::collections::BinaryHeap;
use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};

const _MAX_N: usize = 2e5 as usize;
const MAX_M: usize = 1e9 as usize;

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str(tokens: &mut SplitAsciiWhitespace) -> Vec<char> {
    tokens.next().unwrap().chars().collect()
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let x: usize = next(tokens);
    let a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();

    // 二分求 k 最小值
    // 判断当前 k 是否合法
    let check = |k: usize| -> bool {
        // 孤立标记
        let mut bad = vec![false; n];
        // 孤立点数量
        let mut bad_cnt = 0usize;

        for idx in 0..n {
            let bad_left = (idx == 0) || a[idx - 1] + k < a[idx];
            let bad_right = (idx == n - 1) || a[idx] + k < a[idx + 1];
            if bad_left && bad_right {
                bad[idx] = true;
                bad_cnt += 1;
            }
        }

        // 计算能被同时满足的点的数量
        let mut pair = 0usize;
        let mut idx = 0;
        while idx < n {
            if bad[idx] && idx + 1 < n && bad[idx + 1] && a[idx + 1] - a[idx] <= 2 * k {
                pair += 1;
                idx += 2;
            } else {
                idx += 1;
            }
        }

        let needed = bad_cnt - pair;

        needed <= x
    };

    let mut l = 1usize;
    let mut r = MAX_M;
    while l < r {
        let mid = (l + r) >> 1;
        if check(mid) {
            r = mid;
        } else {
            l = mid + 1;
        }
    }

    println!("{l}");
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
