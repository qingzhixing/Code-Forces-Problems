use std::io::{self, Read};
use std::println;
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
    let x: usize = next(tokens);

    // 给定步数求最大伤害
    let max_damage = |step: usize| -> usize {
        let mut best = 0usize;
        for atk in 1..=step {
            // 充能次数
            let power = step - atk;
            let damage = if atk <= power + 1 {
                // 花不完
                atk * (2 * power + 3 - atk) / 2
            } else {
                (power + 1) * (power + 2) / 2 + (atk - power - 1)
            };

            best = best.max(damage);
        }
        best
    };

    let mut step = 1usize;
    while max_damage(step) < x {
        step += 1;
    }

    println!("{step}");
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
