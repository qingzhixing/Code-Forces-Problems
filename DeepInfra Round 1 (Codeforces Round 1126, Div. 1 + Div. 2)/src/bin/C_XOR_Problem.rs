use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};
use std::{print, println};

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
    let msb = 1usize << n.ilog2();

    // 构造前缀异或和序列
    let mut pre_xor = Vec::<usize>::new();
    // 第一组 0 ~ msb - 1
    for i in 0..msb {
        pre_xor.push(i);
    }
    for i in (0..msb).rev() {
        pre_xor.push(i);
    }
    // 第二组 msb ~ 2 * msb - 1
    for i in msb..(2 * msb) {
        pre_xor.push(i);
    }
    for i in (msb..(2 * msb)).rev() {
        pre_xor.push(i);
    }

    // 原数组长度 = 前缀异或数组长度 - 1
    let k = pre_xor.len() - 1;
    println!("{}", k);

    // a_i = p[i] ^ p[i-1]，i 从 1 到 k
    for i in 1..=k {
        if i > 1 {
            print!(" ");
        }
        print!("{}", pre_xor[i] ^ pre_xor[i - 1]);
    }
    println!();
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
