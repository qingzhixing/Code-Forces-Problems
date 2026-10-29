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
    let m: usize = next(tokens);

    let mut degree = vec![0usize; n];

    for _ in 0..m {
        let mut u: usize = next(tokens);
        let mut v: usize = next(tokens);

        // 0 - indexed
        u -= 1;
        v -= 1;

        degree[u] += 1;
        degree[v] += 1;
    }

    degree.sort_unstable();

    // 存在孤立的点则无解
    if degree[0] == 0 {
        println!("Brain Melt");
        return;
    }

    // 若是星图，则有两个解
    if degree[n - 2] == 1 && degree[n - 1] == n - 1 {
        println!("2");
        return;
    }

    // 其他情况一定有两个以上的解
    println!("Many");
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
