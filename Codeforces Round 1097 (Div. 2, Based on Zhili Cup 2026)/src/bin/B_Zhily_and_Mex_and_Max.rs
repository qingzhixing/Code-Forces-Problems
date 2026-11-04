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
    let mut a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();
    a.sort_unstable();

    let mut result = 0usize;

    // 构造 max_a 放最前，后续数字从小到大排
    let max_a = a.pop().unwrap();
    result += n * max_a;

    let mut unique = Vec::new();
    let mut duplicate = Vec::new();
    let mut previous = None;
    for &num in &a {
        if Some(num) != previous {
            unique.push(num);
            previous = Some(num);
        } else {
            duplicate.push(num);
        }
    }

    // mex 最多为 n + 1
    let mut visited = vec![false; n + 2];
    let mut current_mex = 0usize;

    let mut add_to_mex = |num: usize| {
        if num < visited.len() {
            visited[num] = true;
        }
        while visited[current_mex] {
            current_mex += 1;
        }

        result += current_mex;
    };

    // 最大值放最前面
    add_to_mex(max_a);

    // 放非重复值，实现 mex 最快增长
    for num in unique {
        add_to_mex(num);
    }

    // 放重复值
    for num in duplicate {
        add_to_mex(num);
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
