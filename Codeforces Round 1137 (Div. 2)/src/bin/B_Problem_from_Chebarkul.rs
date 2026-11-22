use std::collections::{BTreeMap, VecDeque};
use std::io::{self, Read};
use std::ops::AddAssign;
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
    let n: usize = next(tokens);
    let k: usize = next(tokens);
    let a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();

    // 维护区间长
    let size = n - k + 1;
    let mut result = 0usize;
    let mut queue = VecDeque::new();
    let mut freq = BTreeMap::<usize, usize>::new();
    for num in a {
        queue.push_back(num);
        freq.entry(num).or_insert(0).add_assign(1);
        if queue.len() >= size {
            if freq[&num] >= size {
                result += 1;
            }
            while queue.len() >= size {
                let deleted = queue.pop_front().unwrap();
                *freq.get_mut(&deleted).unwrap() -= 1;
            }
        }
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
