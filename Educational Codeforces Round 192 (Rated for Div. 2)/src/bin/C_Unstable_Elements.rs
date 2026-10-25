use std::io::{self, Read};
use std::println;
use std::str::{FromStr, SplitAsciiWhitespace};

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
    let n: i64 = next(tokens);
    let k: i64 = next(tokens);
    let a: Vec<i64> = (0..n).map(|_| next(tokens)).collect();

    // 各个分块大小集合
    let mut sizes = Vec::new();
    let mut idx = 0usize;
    while idx < n as usize {
        let mut unequal = idx + 1;

        // 移动到不相等的位置
        while unequal < n as usize && a[idx] == a[unequal] {
            unequal += 1;
        }

        sizes.push((unequal - idx) as i64);
        idx = unequal;
    }
    sizes.sort_unstable();

    let m = sizes.len();
    let mut result = 0i64;

    // 当前保留块数
    let mut q = m as i64;
    // 保留块的大小和
    let mut sum = n;

    // 从小到大枚举删除的块
    let mut size_idx = 0usize;
    while size_idx < m {
        // 保留块中最小的大小
        let s = sizes[size_idx];

        // 删除 s - 1 次
        if (k - sum) % q == 0 && k >= sum - q * (s - 1) {
            result += 1;
        }

        // 删除所有 == s 的块，为下一次枚举做准备
        let mut unequal = size_idx;
        while unequal < m && sizes[unequal] == s {
            unequal += 1;
        }
        let cnt = (unequal - size_idx) as i64;
        q -= cnt;
        sum -= s * cnt;
        size_idx = unequal;
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
