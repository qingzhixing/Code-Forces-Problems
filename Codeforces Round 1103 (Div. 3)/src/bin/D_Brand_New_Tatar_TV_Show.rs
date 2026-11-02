use std::collections::HashMap;
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
    let k: i64 = next(tokens);
    let a: Vec<i64> = (0..n).map(|_| next(tokens)).collect();

    // 计算频率
    let mut freq: HashMap<i64, usize> = HashMap::new();
    for &num in &a {
        *freq.entry(num).or_insert(0) += 1;
    }
    let mut freq: Vec<(i64, usize)> = freq.into_iter().collect();
    freq.sort_unstable_by(|a, b| b.0.cmp(&a.0)); // 按值降序

    let mut last_num = 1e9 as i64;
    for (num, freq) in freq {
        // 当前数字与前一个之差 > k
        if last_num - num > k {
            // 当前值为当前分块内最大的数
            // 如果当前值频率为偶数，则必赢
            if freq % 2 == 0 {
                println!("YES");
                return;
            }
        } else {
            // 当前值非最大值，最大值为奇数
            // 我们只会进入当前分支一次，当前值为当前分块第二大的值
            // 存在第二大值那么也是必赢
            println!("YES");
            return;
        }
        last_num = num;
    }
    println!("NO");
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
