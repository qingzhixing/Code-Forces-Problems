use std::collections::BinaryHeap;
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
    let m: usize = next(tokens);
    let a: Vec<i64> = (0..n).map(|_| next(tokens)).collect();

    let mut result = -1e16 as i64;
    // 选取前 m - 1 最小值 与 当前值组成序列
    let mut heap: BinaryHeap<i64> = BinaryHeap::new();
    // heap 中元素的和
    let mut heap_sum = 0i64;
    for num in a {
        // 还没满
        if heap.len() < m - 1 {
            heap.push(num);
            heap_sum += num;
        } else {
            // 满了则可以尝试求解答案
            result = result.max(num * m as i64 - heap_sum);
            if let Some(&max_value) = heap.peek() {
                // 如果当前值比 max_value 还要小，那么就替换
                if num < max_value {
                    heap_sum -= max_value;
                    heap.pop();
                    heap.push(num);
                    heap_sum += num;
                }
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
