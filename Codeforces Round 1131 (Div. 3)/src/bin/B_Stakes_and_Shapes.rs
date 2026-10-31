use std::io::{self, Read};
use std::print;
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
    let x: usize = next(tokens);
    let y: usize = next(tokens);
    let z: usize = next(tokens);

    let max = x.max(y.max(z));
    let win_ch;
    if max == x {
        win_ch = 'P';
    } else if max == y {
        win_ch = 'R';
    } else {
        win_ch = 'S';
    }

    for _ in 0..x + y + z {
        print!("{win_ch}");
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
