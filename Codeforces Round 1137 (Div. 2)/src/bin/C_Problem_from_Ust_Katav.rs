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
    let p: Vec<usize> = (0..n).map(|_| next(tokens)).collect();

    // 前后第一个比当前点小的点的坐标与当前点的坐标差
    let mut left_dist = vec![0usize; n];
    let mut right_dist = vec![0usize; n];

    let mut stack = Vec::with_capacity(n);
    for (i, &x) in p.iter().enumerate() {
        while let Some(&prev_idx) = stack.last() {
            if p[prev_idx] > x {
                stack.pop();
            } else {
                break;
            }
        }
        left_dist[i] = match stack.last() {
            Some(&prev_idx) => i - prev_idx,
            None => i + 1,
        };

        stack.push(i);
    }
    let left_dist = left_dist;

    stack.clear();

    for (i, &x) in p.iter().enumerate().rev() {
        while let Some(&prev_idx) = stack.last() {
            if p[prev_idx] > x {
                stack.pop();
            } else {
                break;
            }
        }
        right_dist[i] = match stack.last() {
            Some(&prev_idx) => prev_idx - i,
            None => n - i,
        };

        stack.push(i);
    }
    let right_dist = right_dist;

    let mut result = 0usize;
    // 枚举每个点的左右边界情况
    for i in 0..n {
        let left = left_dist[i];
        let right = right_dist[i];
        if left == 1 || right == 1 {
            result += left + right - 1;
        } else {
            result += 2;
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
