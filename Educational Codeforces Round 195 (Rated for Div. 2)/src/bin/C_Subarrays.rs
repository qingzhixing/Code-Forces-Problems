use std::io::{self, Read};
use std::iter;
use std::str::{FromStr, SplitAsciiWhitespace};

const _MAX_N: usize = 0;
const _MAX_M: usize = 0;

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str(tokens: &mut SplitAsciiWhitespace) -> Vec<char> {
    tokens.next().unwrap().chars().collect()
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let k: usize = next(tokens);
    let a: Vec<usize> = iter::once(0).chain((0..n).map(|_| next(tokens))).collect();
    let presum_mod: Vec<usize> = a
        .into_iter()
        .scan(0usize, |prev, value| {
            *prev = (*prev + value) % k;
            Some(*prev)
        })
        .collect();

    let mut result = 0usize;
    // cnt[i] 距离 r 距离 > k 的 模为 i 的 前缀和数量
    let mut cnt = vec![0usize; k + 1];
    // 不断向右枚举 区间右端点
    for r in k + 1..=n {
        // 新进入该滑动窗口的左端点
        let new_l = r - k - 1;
        cnt[presum_mod[new_l]] += 1;
        // 当前右端点能产生的贡献
        result += cnt[presum_mod[r]];
    }

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
