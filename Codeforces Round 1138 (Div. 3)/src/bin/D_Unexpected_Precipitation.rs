use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};
use std::{print, println, unreachable, vec};

#[allow(dead_code)]
fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str(tokens: &mut SplitAsciiWhitespace) -> Vec<char> {
    tokens.next().unwrap().chars().collect()
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    compile_error!("TODO: Wrong Answer.");
    let n: usize = next(tokens);
    let x: Vec<i64> = (0..n).map(|_| next(tokens)).collect();
    let y: Vec<i64> = (0..n).map(|_| next(tokens)).collect();

    // i ~ n - 1 的后缀和
    let mut x_suff_sum = vec![0i64; n + 1];
    let mut y_suff_sum = vec![0i64; n + 1];
    for i in (0..n).rev() {
        x_suff_sum[i] = x_suff_sum[i + 1] + x[i];
        y_suff_sum[i] = y_suff_sum[i + 1] + y[i];
    }

    if x_suff_sum[0] > y_suff_sum[0] {
        println!("Sunny");
        return;
    }

    let mut sunny_left = true;
    let mut sunny_pay = 0i64;
    let mut aubrey_pay = 0i64;
    for i in 0..n {
        let sunny = if sunny_left { &x_suff_sum } else { &y_suff_sum };
        let aubrey = if sunny_left { &y_suff_sum } else { &x_suff_sum };

        let sunny_current = sunny[i] - sunny[i + 1];
        let aubrey_current = aubrey[i] - aubrey[i + 1];

        // 当前购入收益
        let buy_delta = sunny_current - aubrey_current;
        let swap_delta = aubrey[i + 1] - sunny[i + 1] - buy_delta;

        print!("sunny_left: {sunny_left}, buy_delta: {buy_delta}, swap_delta:{swap_delta} ");

        if (swap_delta > 0) == (sunny_left) {
            println!("swap better");
            // 交换对当前左侧更优
            sunny_left = !sunny_left;
        } else {
            println!("buy better");
            // 不交换对当前左侧最优
            sunny_pay += sunny[i] - sunny[i + 1];
            aubrey_pay += aubrey[i] - aubrey[i + 1];
        }
    }

    let d = sunny_pay - aubrey_pay;
    match d {
        d if d < 0 => {
            println!("Aubrey");
        }
        d if d == 0 => {
            println!("Draw");
        }
        d if d > 0 => {
            println!("Sunny");
        }
        _ => unreachable!(),
    }
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
