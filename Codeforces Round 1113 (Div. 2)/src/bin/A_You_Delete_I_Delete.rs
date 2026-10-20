use std::io::{self, Read, Write};
use std::str::{FromStr, SplitAsciiWhitespace};

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

fn solution(tokens: &mut SplitAsciiWhitespace, out: &mut impl Write) {
    let s: Vec<char> = tokens.next().unwrap().chars().collect();
    let mut first0 = true;
    let mut first1 = true;
    s.iter().for_each(|ch| {
        if *ch == '1' {
            if !first1 {
                write!(out, "{ch}").unwrap();
            }
            first1 = false;
        } else {
            if !first0 {
                write!(out, "{ch}").unwrap();
            }
            first0 = false;
        }
    });
    writeln!(out, "").unwrap();
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
