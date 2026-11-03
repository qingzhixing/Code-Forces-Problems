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

fn lowbit(num: usize) -> usize {
    num & num.wrapping_neg()
}

fn all_meat(num: usize) -> bool {
    let mut cnt = 0usize;
    let mut num = num;
    while num > 0 {
        num -= lowbit(num);
        cnt += 1;
    }
    cnt % 2 == 0
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let q: usize = next(tokens);

    let mut a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();
    let mut result = a.iter().filter(|&&num| all_meat(num)).count();
    print!("{result} ");
    for _ in 0..q {
        let p: usize = next(tokens);
        let x: usize = next(tokens);

        // 0 - indexed
        let p = p - 1;

        result -= all_meat(a[p]) as usize;
        a[p] = x;
        result += all_meat(x) as usize;

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
