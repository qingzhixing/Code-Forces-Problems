use std::io::{self, Read, Write};
use std::str::{FromStr, SplitAsciiWhitespace};

const _MAX_N: usize = 0;
const _MAX_M: usize = 0;

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str<'a>(tokens: &mut SplitAsciiWhitespace<'a>) -> &'a str {
    tokens.next().unwrap()
}

fn solution(tokens: &mut SplitAsciiWhitespace, out: &mut impl Write) {
    let n: isize = next(tokens);
    let k: isize = next(tokens);
    // 无解
    if n - k == 1 {
        writeln!(out, "-1").unwrap();
        return;
    }

    let block_cnt = n - k;
    let need0 = (n + 1) / 2;
    let need1 = n / 2;
    let cnt0 = (block_cnt + 1) / 2;
    let cnt1 = block_cnt / 2;
    let extra0 = (need0 - cnt0) as usize;
    let extra1 = (need1 - cnt1) as usize;

    for block_id in 0..block_cnt {
        write!(out, "{}", block_id % 2).unwrap();
        if block_id / 2 < 1 {
            if block_id % 2 == 0 {
                write!(out, "{}", "0".repeat(extra0)).unwrap();
            } else {
                write!(out, "{}", "1".repeat(extra1)).unwrap();
            }
        }
    }
    writeln!(out).unwrap();
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
    out.flush().unwrap();
}
