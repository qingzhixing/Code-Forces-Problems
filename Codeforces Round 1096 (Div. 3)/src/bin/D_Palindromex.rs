use std::collections::BTreeSet;
use std::io::{self, Read};
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
    let n = n * 2;
    let a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();
    let pos: Vec<_> = a
        .iter()
        .enumerate()
        .filter(|x| *x.1 == 0)
        .map(|x| x.0)
        .collect();
    let pos0 = pos[0];
    let pos1 = pos[1];

    let is_palindrome = |mut l: usize, mut r: usize| -> bool {
        while l < r {
            if a[l] != a[r] {
                return false;
            }
            if l + 1 >= n || r <= 0 {
                return true;
            }
            l += 1;
            r -= 1;
        }
        true
    };

    let expand_palindrome = |mut l: usize, mut r: usize| -> (usize, usize) {
        loop {
            if l <= 0 || r + 1 >= n {
                return (l, r);
            }
            l -= 1;
            r += 1;
            if a[l] != a[r] {
                return (l + 1, r - 1);
            }
        }
    };

    let get_mex = |l: usize, r: usize| -> usize {
        let mut set = BTreeSet::<usize>::new();
        for i in l..=r {
            set.insert(a[i]);
        }
        let mut mex = 0;
        while set.contains(&mex) {
            mex += 1;
        }
        mex
    };

    if is_palindrome(pos0, pos1) {
        let (l, r) = expand_palindrome(pos0, pos1);
        println!("{}", get_mex(l, r));
        return;
    }

    let mut result = 0usize;
    let (l0, r0) = expand_palindrome(pos0, pos0);
    result = result.max(get_mex(l0, r0));
    let (l1, r1) = expand_palindrome(pos1, pos1);
    result = result.max(get_mex(l1, r1));

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
