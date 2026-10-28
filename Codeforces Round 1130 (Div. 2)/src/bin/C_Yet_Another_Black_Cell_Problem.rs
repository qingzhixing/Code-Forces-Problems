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

    // 二分求 k
    let check = |k: usize| -> bool {
        // 求孤立点数量
        let mut bad = vec![false; n];
        let mut bad_cnt = 0usize;
        for i in 0..n {
            let left_bad = (i == 0) || a[i] - a[i - 1] > k;
            let right_bad = (i == n - 1) || a[i + 1] - a[i] > k;
            if left_bad && right_bad {
                bad[i] = true;
                bad_cnt += 1;
            }
        }

        // 求能被同时满足的两个点
        let mut idx = 0;
        let mut pair = 0usize;
        while idx < n - 1 {
            if bad[idx] && bad[idx + 1] && a[idx + 1] - a[idx] <= 2 * k {
                pair += 1;
                idx += 2;
            } else {
                idx += 1;
            }
        }

        bad_cnt - pair <= x
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
