use std::collections::VecDeque;
use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};
use std::{println, vec};

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
    let mut children: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 1..n {
        // 0 - indexed parent
        let p = next::<usize>(tokens) - 1;
        children[p].push(i);
    }
    let children = children;

    // 求 BFS序 并且 初始化各点深度
    let mut bfs_order = vec![];
    let mut depth = vec![0usize; n];
    // parent, parent_depth
    let mut q = VecDeque::from([(0usize, 0usize)]);
    while let Some((current, parent_depth)) = q.pop_front() {
        bfs_order.push(current);
        depth[current] = parent_depth + 1;
        for &child in &children[current] {
            q.push_back((child, parent_depth + 1));
        }
    }
    let depth = depth;

    // 逆 BFS序 初始化子树深度 并 统计答案
    let mut result = n;
    let mut subtree_depth = vec![0usize; n];
    bfs_order.reverse();
    for current in bfs_order {
        subtree_depth[current] = depth[current];

        // 统计 子节点子树深度
        let mut children_depth = vec![];
        for &child in &children[current] {
            let child_depth = subtree_depth[child];
            subtree_depth[current] = subtree_depth[current].max(child_depth);
            children_depth.push(child_depth);
        }

        // 如果当前深度有两个以上子节点可达则加入统计
        for d in depth[current] + 1..=subtree_depth[current] {
            let cnt = children_depth.iter().filter(|&x| *x >= d).count();
            if cnt < 2 {
                // 可达性递减，剪枝
                break;
            }
            result += 1;
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
