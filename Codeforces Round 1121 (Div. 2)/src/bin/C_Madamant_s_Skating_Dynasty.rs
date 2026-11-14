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

const MOD: usize = 998244353usize;

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let mut a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();

    if n == 1 {
        println!("0");
        return;
    }

    a.sort_unstable();
    a.iter_mut().for_each(|x| *x %= MOD);

    // 需要选择父节点的点的数量
    let m = n - 1;

    // 预处理出前缀后缀阶乘
    // fact_prefix[i] = i!
    let mut fact_prefix = vec![1usize; m + 1];
    for i in 1..=m {
        fact_prefix[i] = (fact_prefix[i - 1] * i) % MOD;
    }
    let fact_prefix = fact_prefix;

    // fact_suffix[i] = (m - i)(m - i + 1)...(m)
    let mut fact_suffix = vec![1usize; m + 2];
    for i in (1..=m).rev() {
        fact_suffix[i] = (fact_suffix[i + 1] * i) % MOD;
    }
    let fact_suffix = fact_suffix;

    let mut result = 0usize;
    let mut suffix_sum = a[n - 1];
    // 从后往前枚举节点
    for i in (0..m).rev() {
        // i 节点可选父亲数量
        let k = m - i;
        let choice = (fact_suffix[k + 1] * fact_prefix[k - 1]) % MOD;
        result =
            (result + (choice * (suffix_sum + MOD - ((a[i] * k) % MOD) % MOD) % MOD) % MOD) % MOD;
        suffix_sum = (suffix_sum + a[i]) % MOD;
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
