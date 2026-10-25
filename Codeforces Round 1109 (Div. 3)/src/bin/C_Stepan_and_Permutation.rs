use std::collections::VecDeque;
use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};
use std::vec;

const _MAX_N: usize = 0;
const _MAX_M: usize = 0;

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str<'a>(tokens: &mut SplitAsciiWhitespace<'a>) -> &'a str {
    tokens.next().unwrap()
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let x: usize = next(tokens);
    let y: usize = next(tokens);
    let a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();

    // 利用 x, y 给数组分类
    let mut types = vec![0i64; n];
    let mut current_type = 0i64;
    for idx in 0..n {
        // 已分类
        if types[idx] != 0 {
            continue;
        }

        // 分类
        // 新建一个类别
        current_type += 1;
        let mut q = VecDeque::new();
        q.push_back(idx);

        while let Some(current) = q.pop_front() {
            let neighbors = [
                current as i64 - x as i64,
                current as i64 + x as i64,
                current as i64 - y as i64,
                current as i64 + y as i64,
            ];
            for idx in neighbors {
                if idx >= 0 && idx < n as i64 {
                    let idx = idx as usize;
                    if types[idx] == 0 {
                        types[idx] = current_type;
                        q.push_back(idx);
                    }
                }
            }
        }
    }

    // 判断每一个环是否都在同一个类型里
    let mut passed = vec![false; n];
    for idx in 0..n {
        // 当前点访问过了
        if passed[idx] == true {
            continue;
        }

        passed[idx] = true;
        let current_type = types[idx];
        let mut nxt = a[idx] - 1;
        while passed[nxt] != true {
            if types[nxt] != current_type {
                println!("NO");
                return;
            }
            passed[nxt] = true;
            nxt = a[nxt] - 1;
        }
    }

    println!("YES");
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
