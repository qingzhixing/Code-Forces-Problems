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
    let n: usize = next(tokens);
    let m: usize = next(tokens);
    let v: Vec<usize> = (0..n).map(|_| next(tokens)).collect();
    let a: Vec<Vec<usize>> = (0..n)
        .map(|_| (0..m).map(|_| next(tokens)).collect())
        .collect();

    // 答案上界
    let mut result = m;
    // 从下到上枚举每一行，维护全局前 m 大值
    let mut best = vec![];
    for i in (0..n).rev() {
        // 将当前行纳入考虑
        best.extend_from_slice(&a[i]);
        best.sort_unstable_by(|x, y| y.cmp(x));
        best.truncate(m);
        // 统计当前行答案
        let mut cnt = 0usize;
        let mut sum = 0usize;
        for best_idx in 0..m {
            sum += best[best_idx];
            cnt += 1;
            if sum >= v[i] {
                result = result.min(cnt);
                break;
            }
        }
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
