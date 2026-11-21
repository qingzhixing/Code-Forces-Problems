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

fn msb(x: usize) -> usize {
    if x == 0 {
        0
    } else {
        1 << x.ilog2()
    }
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let k: usize = next(tokens);
    let rest = k ^ n;

    if rest == 0 {
        println!("YES");
        for i in 1..n {
            print!("{i} ");
        }
        println!("0");
        return;
    }

    if rest <= n - 1 {
        println!("YES");
        for i in 1..n {
            if i == rest {
                continue;
            }
            print!("{i} ");
        }
        println!("0 {rest}");
        return;
    }

    // rest > n - 1
    if msb(rest) > msb(n - 1) {
        println!("NO");
        return;
    }

    let v = rest ^ (n - 1);

    println!("YES");
    for i in 1..n - 1 {
        if i == v {
            continue;
        }
        print!("{i} ");
    }
    println!("0 {} {}", v, n - 1);
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
