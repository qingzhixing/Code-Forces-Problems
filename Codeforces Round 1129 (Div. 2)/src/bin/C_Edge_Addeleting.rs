use std::collections::VecDeque;
use std::io::{self, Read};
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

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let m: usize = next(tokens);

    // 邻接表存边
    let mut adj = vec![Vec::new(); n + 1];
    for _ in 0..n - 1 {
        let u: usize = next(tokens);
        let v: usize = next(tokens);
        adj[u].push(v);
        adj[v].push(u);
    }

    // 计算 n 到每一点 不经过 点1 的距离
    let mut dist = vec![-1i64; n + 1];
    dist[n] = 0;
    let mut q = VecDeque::new();
    q.push_back(n);

    while let Some(current) = q.pop_front() {
        // 不扩展 1
        if current == 1 {
            continue;
        }

        for &nxt in &adj[current] {
            // 未初始化
            if dist[nxt] < 0 {
                dist[nxt] = dist[current] + 1;
                q.push_back(nxt);
            }
        }
    }

    let mut result = dist[1];
    for _ in 0..m {
        let v: usize = next(tokens);
        // 距离合法
        if dist[v] >= 0 {
            result = result.max(1 + dist[v]);
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
