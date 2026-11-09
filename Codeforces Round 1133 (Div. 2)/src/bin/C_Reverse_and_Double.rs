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
    let a: Vec<i64> = (0..n).map(|_| next(tokens)).collect();
    let diff: Vec<i64> = a.windows(2).map(|w| (w[1] - w[0]).abs()).collect();
    let max_diff = diff.iter().max().unwrap();

    // 求最大值的所有坐标
    let mut max_diff_idx: Vec<usize> = vec![];
    diff.iter().enumerate().for_each(|(idx, x)| {
        if x == max_diff {
            max_diff_idx.push(idx);
        }
    });
    let max_diff_idx = max_diff_idx;

    // 如果当前总数 >= 3，那么我们没法改变最小 gap
    if max_diff_idx.len() >= 3 {
        println!("{max_diff}");
        return;
    }

    // 前后缀最大值
    let pre_max: Vec<_> = diff
        .iter()
        .scan(0i64, |prev, value| {
            if value > prev {
                *prev = *value;
            }
            Some(*prev)
        })
        .collect();
    let suf_max: Vec<_> = diff
        .iter()
        .rev()
        .scan(0i64, |prev, value| {
            if value > prev {
                *prev = *value;
            }
            Some(*prev)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    // 评估区间翻转产生的最值
    let eval = |l: usize, r: usize, inner_max: i64| -> i64 {
        // 区间内部最大值变为两倍
        let mut result = inner_max * 2;

        // 左侧判断
        if l > 0 {
            if l > 1 {
                // 左侧未受影响的完整 gap 到 l-2 为止
                result = result.max(pre_max[l - 2]);
            }
            // 交界处产生新的最大值
            result = result.max((a[l - 1] - a[r] * 2).abs());
        }

        // 右侧判断
        if r < n - 1 {
            if r < n - 2 {
                // 右侧未受影响的完整 gap 从 r+1 开始
                result = result.max(suf_max[r + 1]);
            }
            // 交界处产生新的最大值
            result = result.max((a[r + 1] - a[l] * 2).abs());
        }

        result
    };

    let mut result = *max_diff;

    // 只有两个最大值的情况
    // 操作区间确定，为 [i + 1, j]
    if max_diff_idx.len() == 2 {
        let i = max_diff_idx[0];
        let j = max_diff_idx[1];
        let l = i + 1;
        let r = j;

        let mut inner_max = 0i64;
        for k in l..r {
            inner_max = inner_max.max(diff[k]);
        }

        result = result.min(eval(l, r, inner_max));
    }

    // 只有一个最大值，需要枚举左右侧所有操作可能性
    if max_diff_idx.len() == 1 {
        let i = max_diff_idx[0];
        // 候选情况 A：右边界固定在 i，左边界 l 向左扩展
        let mut inner_max = 0;
        for l in (0..=i).rev() {
            // 边遍历边求最大值
            if l < i {
                inner_max = inner_max.max(diff[l]);
            }
            result = result.min(eval(l, i, inner_max));
        }

        // 候选情况 B：左边界固定在 i + 1，右边界 r 向右扩展
        let mut inner_max = 0;
        for r in i + 1..n {
            // 边遍历边求最大值
            if r > i + 1 {
                inner_max = inner_max.max(diff[r - 1]);
            }
            result = result.min(eval(i + 1, r, inner_max));
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
