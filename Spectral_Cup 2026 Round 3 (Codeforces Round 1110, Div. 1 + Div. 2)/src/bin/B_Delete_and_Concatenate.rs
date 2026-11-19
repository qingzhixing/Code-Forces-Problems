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
    let c: i64 = next(tokens);
    let mut a: Vec<_> = (0..n).map(|_| next::<i64>(tokens) - c).collect();

    let k = n.div_ceil(2);
    let not_neg_cnt = a.iter().filter(|&&x| x >= 0).count();
    if not_neg_cnt >= k {
        println!("{}", a.into_iter().filter(|&x| x >= 0).sum::<i64>());
        return;
    }

    a.select_nth_unstable_by(k - 1, |x, y| y.cmp(x));
    a.truncate(k);
    println!("{}", a.into_iter().sum::<i64>());
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
