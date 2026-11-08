use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};
use std::{print, println};

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
    let k: usize = next(tokens);
    let mut a: Vec<i64> = (0..n).map(|_| next(tokens)).collect();
    a.sort_unstable();

    // 判断是否无解
    let max_a = a.last().unwrap();
    let max_cnt = a.iter().filter(|&num| num == max_a).count();
    if k < max_cnt {
        println!("NO");
        return;
    }

    a[..n - k].reverse();
    a.rotate_left(n - k);

    println!("YES");
    for num in a {
        print!("{num} ");
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
