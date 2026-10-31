use std::io::{self, Read};
use std::println;
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
    let first_1 = a[0] == 1;
    let last_1 = a[n - 1] == 1;
    let mid_all_1 = !a[1..n - 1].iter().any(|&x| x != 1);

    if first_1 && mid_all_1 && last_1 {
        println!("-1");
        return;
    }

    let sum: usize = a.iter().sum();
    if mid_all_1 {
        // 中间全是 1, 那么答案只能由首尾贡献
        println!("{}", a[0] + a[n - 1] - 1);
    } else {
        // 中间不全是 1，那么中间能全部消掉而且不产生损失，损失只有首尾合并的一次
        println!("{}", sum - 1);
    }
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
