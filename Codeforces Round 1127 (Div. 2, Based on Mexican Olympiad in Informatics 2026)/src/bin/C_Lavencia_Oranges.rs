use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};

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

    // 二分求解最小成本
    // 判断当前成本是否可行
    let check = |cost: usize| -> bool {
        let mut capacity = 0usize;
        for size in 1..=cost {
            capacity += (cost / size) * size;
        }
        capacity >= n
    };

    let mut l = 0usize;
    let mut r = n;
    while l < r {
        let mid = (l + r) >> 1;
        if check(mid) {
            r = mid;
        } else {
            l = mid + 1;
        }
    }

    // 根据求解的最小成本构造答案
    let cost = l;
    let mut capacity = n;
    let mut sizes = Vec::<usize>::new();

    'outer: for size in (1..=cost).rev() {
        for _ in 0..cost / size {
            if capacity >= size {
                capacity -= size;
                sizes.push(size);
            } else {
                if capacity != 0  {
                    sizes.push(capacity);
                }
                break 'outer;
            }
        }
    }

    println!("{}", cost);
    for (idx, &size) in sizes.iter().enumerate() {
        for _ in 0..size {
            print!("{} ", idx + 1);
        }
    }
    println!("");
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
