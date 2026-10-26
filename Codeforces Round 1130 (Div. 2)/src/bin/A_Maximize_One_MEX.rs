use std::collections::HashSet;
use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};

const _MAX_N: usize = 0;
const _MAX_M: usize = 0;

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str<'a>(tokens: &mut SplitAsciiWhitespace<'a>) -> &'a str {
    tokens.next().unwrap()
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let k: usize = next(tokens);
    let a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();
    let mut num_to_idx = vec![0usize; n + 10];

    let mut set = HashSet::<usize>::new();

    // 求前 n - k + 1 个数的 mex , 并初始化 num_to_idx;
    let mut max_idx = 0;
    for idx in 0..n {
        // 1 - indexed
        num_to_idx[a[idx]] = idx + 1;
        if idx <= n - k {
            if a[idx] > a[max_idx] {
                max_idx = idx;
            }
            set.insert(a[idx]);
        }
    }

    let mut mex = 0;
    while set.contains(&mex) {
        mex += 1;
    }

    // 如果已经是理论最大 mex 则不需要 op
    if mex == n - k + 1 {
        println!("-1");
        return;
    }

    // 否则交换 mex 和 窗口内最大数字
    println!("{} {}", num_to_idx[mex], max_idx + 1);
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
