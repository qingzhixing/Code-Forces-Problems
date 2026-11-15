use std::collections::BTreeSet;
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
    let s: Vec<usize> = next_str(tokens)
        .into_iter()
        .map(|x| x as usize - '0' as usize)
        .collect();
    let mut set = BTreeSet::<Vec<usize>>::new();
    set.insert(s.clone());
    for l in 0..n {
        for r in l..n {
            for x in 0..=1 {
                let mut s = s.clone();
                for idx in l..=r {
                    s[idx] = x;
                }
                set.insert(s);
            }
        }
    }
    println!("{}", set.len());
    set.iter().for_each(|s| {
        for x in s {
            print!("{x} ");
        }
        println!("");
    });
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
