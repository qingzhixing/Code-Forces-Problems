use std::collections::VecDeque;
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
    compile_error!("TODO: Time Limit Error.");
    let n: usize = next(tokens);
    let p: Vec<usize> = (0..n).map(|_| next(tokens)).collect();

    // 答案下界
    let mut result = n;
    if n > 1 {
        result += 1;
    }

    let mut min = 1e9 as usize;
    let mut min_idx = 0usize;
    for (i, &x) in p.iter().enumerate() {
        if x < min {
            min = x;
            min_idx = i;
        }
    }

    let max_size = (n - 1).min((min_idx + 1).max(n - min_idx));

    // 枚举区间长度
    for len in (2..=max_size).rev() {
        // 滑动窗口维护区间最小值
        let mut q = VecDeque::<usize>::new();
        let mut min = vec![0usize; n];
        for i in 0..n {
            // 插入当前元素
            while !q.is_empty() && p[*q.back().unwrap()] >= p[i] {
                q.pop_back();
            }
            q.push_back(i);

            if i >= len - 1 {
                // 弹出超时元素
                while *q.front().unwrap() + len <= i {
                    q.pop_front();
                }

                let current_min = *q.front().unwrap();
                min[i] = current_min;
            }
        }

        // 统计答案
        for i in len - 1..n {
            let left_ok = i == (len - 1) || min[i - 1] != min[i];
            let right_ok = i == n - 1 || min[i + 1] != min[i];
            if left_ok && right_ok {
                result += 1;
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
