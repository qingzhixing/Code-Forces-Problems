use std::cmp::max;
use std::io::{self, Read, Write};
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

fn solution(tokens: &mut SplitAsciiWhitespace, out: &mut impl Write) {
    let n: usize = next(tokens);
    let a: Vec<i64> = std::iter::once(0i64)
        .chain((0..2 * n).map(|_| next(tokens)))
        .collect();

    // 前缀 1 ~ i 的最大得分
    let mut dp = vec![0i64; 2 * n + 1];
    // 数字 i 第一次出现的位置
    let mut first_idx = vec![-1isize; n + 1];

    for idx in 1..=2 * n {
        // 自成一个区间
        dp[idx] = max(dp[idx], dp[idx - 1] + 1);
        let l = &mut first_idx[a[idx] as usize];
        if *l != -1 {
            let l = first_idx[a[idx] as usize] as usize;
            dp[idx] = max(
                dp[idx],
                dp[(l - 1) as usize] + ((idx - l + 1) * (idx - l + 1)) as i64,
            );
        } else {
            *l = idx as isize;
        }
    }

    writeln!(out, "{}", dp[2 * n]).unwrap();
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());

    let t: usize = next(&mut tokens);
    for _ in 0..t {
        solution(&mut tokens, &mut out);
    }
    out.flush().unwrap();
}
