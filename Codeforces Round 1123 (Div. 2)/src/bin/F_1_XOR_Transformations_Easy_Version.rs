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

fn transform(a: &Vec<usize>) -> Vec<usize> {
    let mut result = vec![];
    let len = a.len();
    for i in 0..len - 1 {
        for j in i + 1..len {
            result.push(a[i] ^ a[j]);
        }
    }
    result.select_nth_unstable(len - 1);
    result.truncate(len);
    result
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let q: usize = next(tokens);
    let mut a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();
    let mut transform_ans = vec![];
    // transform 0
    transform_ans.push(a.iter().max().unwrap() - a.iter().min().unwrap());
    loop {
        a = transform(&a);
        let delta = a.iter().max().unwrap() - a.iter().min().unwrap();
        transform_ans.push(delta);
        if delta <= 0 {
            break;
        }
    }
    let transform_ans = transform_ans;
    drop(a);

    // 收敛次数
    let convergence = transform_ans.len() - 1;

    for _ in 0..q {
        let x: usize = next(tokens);
        if x <= convergence {
            println!("{}", transform_ans[x]);
        } else {
            println!("0");
        }
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
