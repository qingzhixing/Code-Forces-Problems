use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};
use std::{println, vec};

#[allow(dead_code)]
fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str(tokens: &mut SplitAsciiWhitespace) -> Vec<char> {
    tokens.next().unwrap().chars().collect()
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let _n: usize = next(tokens);
    let _m: usize = next(tokens);
    let a = next_str(tokens);
    let b = next_str(tokens);

    let mut cnt_a = vec![0usize; 30];
    let mut cnt_b = vec![0usize; 30];

    for ch in a {
        cnt_a[ch as usize - 'a' as usize] += 1;
    }

    for ch in b {
        cnt_b[ch as usize - 'a' as usize] += 1;
    }

    for i in 0..26 {
        if cnt_a[i] < cnt_b[i] {
            println!("NO");
            return;
        } else {
            cnt_a[i] -= cnt_b[i];
        }
    }

    let mut success = cnt_a['s' as usize - 'a' as usize] == 1
        && cnt_a['i' as usize - 'a' as usize] == 1
        && cnt_a['x' as usize - 'a' as usize] == 1;
    for i in 0..26 {
        let ch = (i + 'a' as u8) as char;
        if ch == 's' || ch == 'i' || ch == 'x' {
            continue;
        }
        if cnt_a[i as usize] != 0 {
            success = false;
            break;
        }
    }

    if success {
        println!("YES");
    } else {
        println!("NO");
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
