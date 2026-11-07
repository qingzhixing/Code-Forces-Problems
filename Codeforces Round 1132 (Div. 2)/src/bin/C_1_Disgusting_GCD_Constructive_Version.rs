use std::io::{self, BufWriter, Read, Write};
use std::str::{FromStr, SplitAsciiWhitespace};
use std::sync::LazyLock;
use std::vec;

compile_error!("TODO: TLE");

const MAX_M: usize = 6e5 as usize;

// 预处理 2 ~ MAX_M 的素数
const PRIMES: LazyLock<Vec<usize>> = LazyLock::new(|| -> Vec<_> {
    let mut primes: Vec<usize> = vec![];
    let mut is_prime = vec![true; MAX_M];
    for num in 2..MAX_M {
        if is_prime[num] {
            primes.push(num);
        }

        // 用当前数字和已保存的质数筛后续数字
        for &prime in &primes {
            if prime * num >= MAX_M {
                break;
            }
            // prime 为 prime * num 的最小质因子
            is_prime[prime * num] = false;
            if num % prime == 0 {
                break;
            }
        }
    }
    primes
});

#[allow(dead_code)]
fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str(tokens: &mut SplitAsciiWhitespace) -> Vec<char> {
    tokens.next().unwrap().chars().collect()
}

fn solution(tokens: &mut SplitAsciiWhitespace, out: &mut impl Write) {
    let n: usize = next(tokens);
    let a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();
    let result: Vec<usize> = a
        .into_iter()
        .map(|num| PRIMES[PRIMES.partition_point(|&x| x < num)])
        .collect();
    for item in result {
        write!(out, "{item} ").unwrap();
    }
    writeln!(out, "").unwrap();
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut tokens = input.split_ascii_whitespace();

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let t: usize = next(&mut tokens);
    for _ in 0..t {
        solution(&mut tokens, &mut out);
    }
}
