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
    let a: Vec<i64> = (0..n).map(|_| next(tokens)).collect();

    // 差分数组，后续求前缀和用于标记无宝藏点
    let mut no_treasure = vec![0i64; n + 1];
    for (i, &r) in a.iter().enumerate() {
        if r <= 0 {
            continue;
        }
        let left = i.saturating_sub(r as usize - 1);
        let right = (n - 1).min(i + r as usize - 1) + 1;
        no_treasure[left] += 1;
        no_treasure[right] -= 1;
    }
    let no_treasure: Vec<_> = no_treasure
        .into_iter()
        .scan(0i64, |prev, x| {
            *prev += x;
            Some(*prev)
        })
        .collect();

    // 判断无解：统一检查所有 x >= 0（含 x == 0）
    for (i, &x) in a.iter().enumerate() {
        if x < 0 {
            continue;
        }
        let x = x as usize;
        // 左端点 i - x 是否合法且未被禁放
        let left_ok = i >= x && no_treasure[i - x] == 0;
        // 右端点 i + x 是否合法且未被禁放
        let right_ok = i + x < n && no_treasure[i + x] == 0;

        if !left_ok && !right_ok {
            println!("-1");
            return;
        }
    }

    let mut result = String::new();
    for (i, x) in a.into_iter().enumerate() {
        if x == 0 {
            result.push('1');
        } else if x != -1 || (x == -1 && no_treasure[i] >= 1) {
            result.push('0');
        } else {
            result.push('1');
        }
    }

    if !result.contains('1') {
        println!("-1");
        return;
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
