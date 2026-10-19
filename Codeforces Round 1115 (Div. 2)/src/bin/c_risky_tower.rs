use std::cmp::min;
use std::io::{self, Read, Write};
use std::str::{FromStr, SplitAsciiWhitespace};

const _MAX_N: usize = 1e6 as usize;
const _MAX_M: usize = 1e9 as usize;

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

fn solution(tokens: &mut SplitAsciiWhitespace, out: &mut impl Write) {
    let n: usize = next(tokens);
    let m: usize = next(tokens);

    let v: Vec<i32> = (0..n).map(|_| next(tokens)).collect();
    let a: Vec<Vec<i32>> = (0..n)
        .map(|_| (0..m).map(|_| next(tokens)).collect())
        .collect();

    // 答案上界
    let mut result = m;

    // 维护前 m 大的项
    let mut best: Vec<i32> = Vec::with_capacity(2 * m);

    for row in (0..n).rev() {
        // 已到达理论最优
        if result == 1 {
            break;
        }

        // 将当前行的数字加入候补
        for col in 0..m {
            best.push(a[row][col]);
        }

        best.sort_unstable_by(|a, b| b.cmp(a));
        best.truncate(m);

        let mut sum = 0;
        for (idx, &val) in best.iter().enumerate() {
            sum += val;
            if sum >= v[row] {
                result = min(result, idx + 1);
                break;
            }
        }
    }

    writeln!(out, "{result}").unwrap();
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
}
