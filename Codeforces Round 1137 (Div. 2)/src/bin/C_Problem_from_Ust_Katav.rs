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

    // 当前点距离左右最小点距离
    let mut left = vec![0usize; n];
    let mut right = vec![0usize; n];

    let mut stack = Vec::with_capacity(n);
    // 单调栈维护前缀更小点
    for i in 0..n {
        while let Some(&back) = stack.last() {
            if p[back] > p[i] {
                stack.pop();
            } else {
                break;
            }
        }
        if let Some(&back) = stack.last() {
            left[i] = i - back;
        } else {
            // 没有更小点，更小点则为 0
            left[i] = i + 1;
        }
        stack.push(i);
    }

    stack.clear();

    // 单调栈维护后缀更小点
    for i in (0..n).rev() {
        while let Some(&back) = stack.last() {
            if p[back] > p[i] {
                stack.pop();
            } else {
                break;
            }
        }
        if let Some(&back) = stack.last() {
            right[i] = back - i;
        } else {
            // 没有更小点，更小点则为 n
            right[i] = n - i;
        }
        stack.push(i);
    }

    // 枚举每个点
    let mut result = 0usize;
    for i in 0..n {
        let left_dist = left[i];
        let right_dist = right[i];

        // 当前最小值与边界相邻
        if left_dist == 1 || right_dist == 1 {
            result += left_dist + right_dist - 1;
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
