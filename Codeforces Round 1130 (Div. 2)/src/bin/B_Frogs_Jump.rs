use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};

compile_error!("TODO: Wrong Answer.");

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
    let line1 = next_str(tokens);
    let line2 = next_str(tokens);
    let mut frogs: Vec<_> = (0..n)
        .map(|idx| {
            let mut cnt = 0usize;
            if line1[idx] == 'f' {
                cnt += 1;
            }
            if line2[idx] == 'f' {
                cnt += 1;
            }
            cnt
        })
        .collect();

    let mut result = 0usize;
    let mut matched = vec![false; n];
    for idx in 0..n {
        if matched[idx] {
            continue;
        }

        // 与右边匹配
        if idx < n - 1 && frogs[idx] == frogs[idx + 1] && frogs[idx] == 1 {
            matched[idx] = true;
            matched[idx + 1] = true;
            continue;
        }

        if frogs[idx] == 1 {
            // 删去多余的 1
            frogs[idx] = 0;
            result += 1;
        }
    }

    println!("{result}");
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
