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
    let mut l = vec![0usize; n];
    let mut r = vec![0usize; n];
    let mut u = vec![0usize; n];
    let mut v = vec![0usize; n];

    for idx in 0..n {
        l[idx] = next(tokens);
        r[idx] = next(tokens);
        u[idx] = next(tokens);
        v[idx] = next(tokens);
    }

    // 从大到小枚举 m 最大值
    for m in (1..=n).rev() {
        // 当前要填的位置
        let mut sub_idx = 1usize;

        // 枚举第 i 个数字看是否能填入当前位置
        for i in 0..n {
            // 填满则退出
            if sub_idx > m {
                break;
            }

            let left_rank = sub_idx;
            let right_rank = m - sub_idx + 1;

            // 左秩 不在 [l, r] 中
            let left_ok = left_rank < l[i] || left_rank > r[i];
            // 右秩 不在 [u, v] 中
            let right_ok = right_rank < u[i] || right_rank > v[i];

            // 满足条件则填入
            if left_ok && right_ok {
                sub_idx += 1;
            }
        }
        // 当前答案合法，就是目标答案
        if sub_idx > m {
            writeln!(out, "{m}").unwrap();
            return;
        }
    }

    writeln!(out, "0").unwrap();
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
