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
    let mut a: Vec<_> = (0..n)
        .map(|idx| (if idx % 2 == 0 { 1 } else { -1 }) * next::<i64>(tokens))
        .collect();

    // 需要分配负号的元素个数正好是 floor(n / 2)
    let k = n / 2;
    if k > 0 {
        a.select_nth_unstable(k);
    }

    let result = a[k..].iter().sum::<i64>() - a[0..k].iter().sum::<i64>();
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
