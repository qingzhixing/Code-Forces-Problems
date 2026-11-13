use std::collections::BTreeMap;
use std::io::{self, Read};
use std::ops::AddAssign;
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
    let mut freqs: BTreeMap<usize, usize> = BTreeMap::new();
    let mut result = 0usize;
    for num in a {
        freqs.entry(num).or_insert(0).add_assign(1);
        if freqs[&num] <= 2 {
            result += 1;
        }
    }
    println!("{result}");
    let mut num_freqs: Vec<(usize, usize)> = freqs.into_iter().collect();
    // 正序输出一遍，之后反序输出所有数字
    for (num, freq) in &mut num_freqs {
        *freq -= 1;
        print!("{num} ");
    }
    for (num, freq) in num_freqs.iter_mut().rev() {
        while *freq > 0 {
            *freq -= 1;
            print!("{num} ");
        }
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
