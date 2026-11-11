use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str(tokens: &mut SplitAsciiWhitespace) -> Vec<char> {
    tokens.next().unwrap().chars().collect()
}

// 极简 Xorshift64 随机数生成器（替代 C++ 的 mt19937）
struct Rng {
    state: u64,
}

impl Rng {
    fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        Self {
            state: if seed == 0 { 0xdeadbeef } else { seed },
        }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    // Fisher-Yates 洗牌算法（替代 C++ 的 std::shuffle）
    fn shuffle<T>(&mut self, slice: &mut [T]) {
        for i in (1..slice.len()).rev() {
            let j = (self.next_u64() as usize) % (i + 1);
            slice.swap(i, j);
        }
    }
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let mut s = next_str(tokens);
    let mut rng = Rng::new();

    let mut result = 3e5 as usize;
    let mut stk = Vec::with_capacity(n);

    for _ in 0..100 {
        rng.shuffle(&mut s);
        stk.clear();

        for &c in &s {
            if let Some(&top) = stk.last() {
                if top != c {
                    stk.pop();
                    continue;
                }
            }
            stk.push(c);
        }

        result = result.min(stk.len());
    }

    println!("{}", result);
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
