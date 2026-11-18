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
    let k: usize = next(tokens);
    let segment = n - k;

    if segment <= 1 {
        println!("-1");
        return;
    }

    let mut last = 1usize;
    let mut second_last = 1usize;
    last += k / 2;
    second_last += k - k / 2;
    for i in 0..segment {
        let ch = ["0", "1"][i & 1];
        print!("{ch}");
        if i == segment - 2 {
            print!("{}", ch.repeat(second_last - 1));
            continue;
        }
        if i == segment - 1 {
            print!("{}", ch.repeat(last - 1));
            continue;
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
