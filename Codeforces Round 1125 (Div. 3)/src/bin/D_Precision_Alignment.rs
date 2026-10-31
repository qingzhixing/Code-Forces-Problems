use std::io::{self, Read};
use std::println;
use std::str::{FromStr, SplitAsciiWhitespace};

const _MAX_N: usize = 2e5 as usize;
const _MAX_M: i64 = 1e18 as i64;

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str(tokens: &mut SplitAsciiWhitespace) -> Vec<char> {
    tokens.next().unwrap().chars().collect()
}

struct Laboratory {
    movable: bool,
    // 想要使得当前实验室评分上升首先需要花费的代价
    prepayment: i64,
    // 当前评分
    rating: i64,
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let k: i64 = next(tokens);
    let mut labs = Vec::<Laboratory>::new();

    for _ in 0..n {
        let a: i64 = next(tokens);
        let b: i64 = next(tokens);
        let c: i64 = next(tokens);

        let movable = !(a == b && b == c);
        let rating = a + b + c;

        let mut prepayment = 0;
        if a <= b && b <= c {
            prepayment = 2 * ((a - b).abs().min((b - c).abs()) + 1);
        }

        labs.push(Laboratory {
            movable,
            prepayment,
            rating,
        });
    }

    // 二分搜索最大的评分
    // 判断当前目标评分能不能达到
    let check = |goal: i64| -> bool {
        let mut remain = k;
        for &Laboratory {
            movable,
            prepayment,
            rating,
        } in &labs
        {
            if rating >= goal {
                continue;
            }
            if !movable {
                return false;
            }
            let needed = prepayment + goal - rating;
            if remain < needed {
                return false;
            }
            remain -= needed;
        }
        true
    };
    let mut l = -_MAX_M;
    let mut r = _MAX_M + k + 10;
    while l < r {
        let mid = (l + r + 1) >> 1;
        if check(mid) {
            l = mid;
        } else {
            r = mid - 1;
        }
    }

    println!("{l}");
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
