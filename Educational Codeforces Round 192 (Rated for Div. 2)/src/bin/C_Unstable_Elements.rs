use std::collections::BTreeMap;
use std::io::{self, Read};
use std::ops::AddAssign;
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
    let a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();

    let mut len_freq = BTreeMap::<usize, usize>::new();
    let mut cnt = 1usize;
    for i in 1..n {
        if a[i] == a[i - 1] {
            cnt += 1;
        } else {
            len_freq.entry(cnt).or_insert(0).add_assign(1);
            cnt = 1;
        }
    }
    // 结算最后一个区间
    len_freq.entry(cnt).or_insert(0).add_assign(1);

    let mut rest_num = n;
    let mut rest_type: usize = len_freq.values().sum();
    let mut result = 0usize;
    for (len, cnt) in len_freq {
        // 保留到当前 len 的最小数组长度
        let min_len = rest_num - (len - 1) * rest_type;
        if k >= min_len && (k - min_len) % rest_type == 0 {
            result += 1;
        }
        rest_num -= len * cnt;
        rest_type -= cnt;
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
