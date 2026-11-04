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
    let a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();
    // 数字为 1 ~ n 的元素的下标的奇偶性
    // 奇数 1 偶数 0
    let mut idx_parity = vec![false; n + 1];
    for (idx, num) in a.into_iter().enumerate() {
        idx_parity[num] = idx % 2 == 1;
    }

    // 从最大值枚举到最小值，对于每个 i ~ n 后缀
    //   操作最后其坐标一定连续，所以操作前
    //   他们的坐标奇数位置和偶数位置之差一定 <= 1
    let mut balance = 0i64;
    for num in (1..=n).rev() {
        balance += if idx_parity[num] { 1 } else { -1 };
        if balance.abs() > 1 {
            println!("NO");
            return;
        }
    }
    println!("YES");
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
