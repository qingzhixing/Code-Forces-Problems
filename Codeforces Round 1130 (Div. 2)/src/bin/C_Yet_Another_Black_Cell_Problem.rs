use std::collections::BinaryHeap;
use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};

compile_error!("TODO: Wrong Answer.");

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
    let mut x: usize = next(tokens);
    let a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();

    let mut heap = BinaryHeap::<usize>::new();
    a.windows(2)
        .into_iter()
        .for_each(|w| heap.push(w[1] - w[0]));

    while x > 0 {
        x -= 1;
        let dist = heap.pop().unwrap();
        heap.push(dist / 2);
        heap.push(dist.div_ceil(2));
    }

    if heap.len() >= 2 {
        heap.pop();
    }
    println!("{}", heap.peek().unwrap());
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
