use std::collections::BTreeMap;
use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};

const _MAX_N: usize = 0;
const _MAX_M: usize = 0;

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

    let mut freq = BTreeMap::<usize, usize>::new();
    for _ in 0..n {
        let a: usize = next(tokens);
        *freq.entry(a).or_insert(0) += 1;
    }

    for num in 0..=n {
        let cnt = *freq.entry(num).or_default();
        if cnt < 2 * k {
            if cnt == 2 * k - 1 {
                println!("YES");
            } else {
                println!("NO");
            }
            return;
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
