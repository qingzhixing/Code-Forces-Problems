use std::cmp::min;
use std::collections::BTreeMap;
use std::io::{self, Read, Write};
use std::str::{FromStr, SplitAsciiWhitespace};

const _MAX_N: usize = 60;
const _MAX_M: usize = 1e3 as usize;

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

fn solution(tokens: &mut SplitAsciiWhitespace, out: &mut impl Write) {
    let n: usize = next(tokens);
    let mut total_sum = 0i64;
    let a: Vec<i64> = (0..n)
        .map(|_| {
            let input: i64 = next(tokens);
            total_sum += input;
            input
        })
        .collect();

    let mut frequncy: BTreeMap<i64, usize> = BTreeMap::new();
    a.into_iter()
        .for_each(|num| *frequncy.entry(num).or_default() += 1);

    let (max_val, max_freq) = frequncy.iter().max_by_key(|&(_k, v)| v).unwrap();
    let other = n - max_freq;

    let result =
        total_sum - max_val * (*max_freq as i64) + max_val * min(other + 2, *max_freq) as i64;
    writeln!(out, "{}", result).unwrap();
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
