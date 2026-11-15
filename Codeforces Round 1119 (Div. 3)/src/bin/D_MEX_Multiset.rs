use std::collections::BTreeMap;
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
    let a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();

    let cnt0 = a.iter().filter(|&&x| x == 0).count();

    if cnt0 == 0 {
        println!("YES");
        println!("{}", "A".repeat(n));
        return;
    }

    if cnt0 == 1 {
        println!("NO");
        return;
    }

    if cnt0 == 2 {
        println!("YES");
        let mut assigned_a = false;
        for x in a {
            if x == 0 {
                if !assigned_a {
                    assigned_a = true;
                    print!("A");
                } else {
                    print!("B");
                }
                continue;
            }
            print!("C");
        }
        println!("");
        return;
    }

    let mut freq = BTreeMap::<usize, usize>::new();
    for &x in &a {
        freq.entry(x).or_insert(0).add_assign(1);
    }

    // 找到最后一个 >= 3 的数字,这就是要构造的最大 mex
    let mut k = 0usize;
    for x in 0..1e9 as usize {
        if *freq.entry(x).or_default() >= 3 {
            k = x;
        } else {
            break;
        }
    }

    println!("YES");
    for x in a {
        if x <= k + 1 {
            if *freq.entry(x).or_default() >= 3 {
                print!("C");
            }
            if *freq.entry(x).or_default() == 2 {
                print!("B");
            }
            if *freq.entry(x).or_default() == 1 {
                print!("A");
            }
            freq.entry(x).and_modify(|x| *x -= 1);
            continue;
        }
        print!("C");
    }
    println!("");
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
