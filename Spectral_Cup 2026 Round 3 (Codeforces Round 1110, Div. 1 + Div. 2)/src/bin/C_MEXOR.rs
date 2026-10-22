use std::io::{self, Read};
use std::iter;
use std::str::{FromStr, SplitAsciiWhitespace};

const _MAX_N: usize = 2e5 as usize + 10;
const _MAX_M: usize = 1e9 as usize;

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str<'a>(tokens: &mut SplitAsciiWhitespace<'a>) -> &'a str {
    tokens.next().unwrap()
}

fn msb_idx(mut number: usize) -> usize {
    let mut result = 0usize;
    while number > 0 {
        number >>= 1;
        result += 1;
    }
    result - 1
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let k: usize = next(tokens);

    if n == 1 {
        if k == 1 {
            println!("YES");
            println!("0");
        } else {
            println!("NO");
        }
        return;
    }

    // 前 n - 1 个数的 mex 一定是 n
    // 前 n - 2 个前缀 mex 的 xor 和则为 k ^ n
    let sub_k = k ^ n;
    if sub_k == 0 {
        // 1 ~ n - 1 放前面，0 放在最后
        println!("YES");
        println!(
            "{}",
            (1..n)
                .into_iter()
                .chain(iter::once(0))
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
        return;
    }
    if sub_k <= n - 1 {
        // 除了 {sub_k, 0} 以外的所有数放前面
        // 0, sub_k 放后面
        println!("YES");
        println!(
            "{}",
            (1..n)
                .into_iter()
                .filter(|&x| x != sub_k)
                .chain(vec![0, sub_k].into_iter())
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
        return;
    }

    // sub_k > n - 1
    let max_msb_idx = msb_idx(n - 1);
    let current_msb_idx = msb_idx(sub_k);
    if current_msb_idx > max_msb_idx {
        // 找不到一个数字 v, 1 <= v <= n-2
        // 使得 v ^ n-1 = sub_k
        println!("NO");
        return;
    }
    // 找到一个数字 v, 1 <= v <= n-2
    // 使得 v ^ n-1 = sub_k
    let v = sub_k ^ (n - 1);

    // 除了 {0, v, n-1} 以外的所有数放前面
    // 0, v, n-1 放后面
    println!("YES");
    println!(
        "{}",
        (1..n)
            .into_iter()
            .filter(|&x| x != v && x != (n - 1))
            .chain(vec![0, v, n - 1].into_iter())
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
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
