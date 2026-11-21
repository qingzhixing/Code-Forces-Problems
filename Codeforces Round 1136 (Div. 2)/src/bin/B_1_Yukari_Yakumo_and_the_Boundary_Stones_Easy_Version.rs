use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};
use std::{panic, println};

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
    let y: usize = next(tokens);
    let m: usize = next(tokens);
    let s = next_str(tokens);

    // 不操作
    let mut result = x + y;

    let max_stones = |prefix_end: usize| -> usize {
        let mut current_x = x;
        let mut current_y = y;
        for i in 0..=prefix_end {
            // 无法继续
            if current_x == 0 && current_y == 0 {
                return 0;
            }

            match s[i] {
                '+' => {
                    if current_y > 0 {
                        current_y += 1;
                    } else {
                        current_x -= 1;
                        current_y += 1;
                    }
                }
                '-' => {
                    if current_y > 1 {
                        current_y -= 1;
                        current_x += 1;
                    } else if current_x > 0 {
                        current_x -= 1;
                    } else {
                        current_y -= 1;
                        current_x += 1;
                    }
                }
                other => {
                    panic!("Error Operator: {}", other);
                }
            }
        }

        current_x + current_y
    };

    for prefix_end in 0..m {
        result = result.max(max_stones(prefix_end));
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
