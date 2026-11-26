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
    let q: usize = next(tokens);
    let s = next_str(tokens);

    // prefix_bad[i] 表示前 i 个相邻对中，满足 s[j] == s[j+1] 的数量
    // 1 - indexed
    let mut prefix_bad = vec![0usize; n + 10];
    for i in 1..n {
        prefix_bad[i] = prefix_bad[i - 1] + (s[i] == s[i - 1]) as usize;
    }

    for _ in 0..q {
        let l: usize = next(tokens);
        let r: usize = next(tokens);
        let k: usize = next(tokens);
        let bad = prefix_bad[r - 1] - prefix_bad[l - 1];
        let k0 = bad.div_ceil(2);
        if k >= k0 {
            println!("YES");
        } else {
            println!("NO");
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
