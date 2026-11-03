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
    let holes: Vec<usize> = (0..n).map(|_| next(tokens)).collect();

    // 从 0 ~ n - 1 枚举空的桶
    for empty_idx in 0..n {
        let mut heights = vec![1e9 as usize + 10; n];
        // 正向限制
        let mut right_limit = 0usize;
        for limit_idx in 0..n {
            let current_idx = (empty_idx + limit_idx) % n;
            heights[current_idx] = heights[current_idx].min(right_limit);
            right_limit = right_limit.max(holes[current_idx]);
        }

		// 反向限制
        let mut left_limit = 0usize;
        for limit_idx in 0..n {
            let current_idx = (empty_idx + n - limit_idx) % n;
            heights[current_idx] = heights[current_idx].min(left_limit);
            left_limit = left_limit.max(holes[(current_idx + n - 1) % n]);
        }

        let result: usize = heights.iter().sum();
        print!("{result} ");
    }
    println!("");
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
