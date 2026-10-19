#![allow(unused)]

use std::io::{self, Read, Write};
use std::str::{FromStr, SplitAsciiWhitespace};

const MAX_N: usize = 0;
const MAX_M: usize = 0;

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

fn solution(tokens: &mut SplitAsciiWhitespace, out: &mut impl Write) {
    let n: usize = next(tokens);
    let s: Vec<char> = tokens.next().unwrap().chars().collect();

    let mut cnt0 = 0i32;
    let mut cnt1 = 0i32;
    let mut last_ch: char = 114 as char;
    let mut reserved: Vec<char> = Vec::new();

    s.into_iter().for_each(|ch| {
        if (last_ch == 114 as char || last_ch != ch) {
            reserved.push(ch);
        } else {
            if (ch == '1') {
                cnt1 += 1;
            } else {
                cnt0 += 1;
            }
        }
        last_ch = ch;
    });

    let mut more0 = 0i32;
    let mut more1 = 0i32;

    if (*reserved.first().unwrap() == '1') {
        more1 += 1;
    } else {
        more0 += 1;
    }

    if (*reserved.last().unwrap() == '1') {
        more1 += 1;
    } else {
        more0 += 1;
    }

    if (cnt0 - cnt1).abs() > 1 {
        // 尝试用首尾字符补足
        while (cnt0 < (cnt1 - 1) && more0 > 0) {
            cnt0 += 1;
            more0 -= 1;
        }

        while (cnt1 < (cnt0 - 1) && more1 > 0) {
            cnt1 += 1;
            more1 -= 1;
        }
    }

    if (cnt0 - cnt1).abs() > 1 {
        writeln!(out, "-1");
    } else {
        writeln!(out, "{}", cnt0 + cnt1);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());

    let t: usize = next(&mut tokens);
    for _ in 0..t {
        solution(&mut tokens, &mut out);
    }
}
