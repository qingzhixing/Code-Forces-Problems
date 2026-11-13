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
    let s = next_str(tokens);
    // [l, r]
    let mut ranges: Vec<(usize, usize)> = vec![];

    // 将不等回文对变为区间
    for i in 0..n {
        if s[i] == s[n - 1 - i] {
            continue;
        }
        let a = s[i] as usize - 'a' as usize;
        let b = s[n - 1 - i] as usize - 'a' as usize;
        let l = a.min(b);
        let r = a.max(b);
        ranges.push((l, r));
    }

    // 将区间按左端点排序
    ranges.sort_unstable();

    let mut l = None::<usize>;
    let mut r = None::<usize>;
    let mut result = 0usize;
    // 区间合并
    for (start, end) in ranges {
        if r != None && r >= Some(start) {
            r = r.max(Some(end));
        } else {
            // 结算上一个区间
            if r != None {
                let l = l.unwrap();
                let r = r.unwrap();
                result += r - l;
            }
            l = Some(start);
            r = Some(end);
        }
    }
    // 结算最后一个区间
    if r != None {
        let l = l.unwrap();
        let r = r.unwrap();
        result += r - l;
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
