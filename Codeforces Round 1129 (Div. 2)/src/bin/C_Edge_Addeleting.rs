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
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    for _ in 0..(n - 1) {
        let u: usize = next(tokens);
        let v: usize = next(tokens);
        adj[u].push(v);
        adj[v].push(u);
    }

    // 初始化 dist
    // dist [i] 表示对结果有影响的点 到 n 的距离
    let mut dist: Vec<Option<usize>> = vec![None; n + 1];
    dist[n] = Some(0);

    // BFS 求 dist
    let mut q: VecDeque<usize> = VecDeque::from([n]);
    while let Some(current) = q.pop_front() {
        // 不扩展 1
        if current == 1 {
            continue;
        }

        let current_dist = dist[current].unwrap();
        for &nxt in &adj[current] {
            if dist[nxt] == None {
                dist[nxt] = Some(current_dist + 1);
                q.push_back(nxt);
            }
        }
    }

    let mut result = dist[1].unwrap();
    for _ in 0..m {
        let v: usize = next(tokens);
        if let Some(dist_v) = dist[v] {
            result = result.max(dist_v + 1);
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
