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

    // gaps[i] = a[i + 1] - a[i]
    // [0..= (n - 2)]
    let gaps: Vec<i64> = a.windows(2).map(|w| (w[1] - w[0]).abs()).collect();

    let max_gap = gaps.iter().max().unwrap();
    let max_gap_idxs: Vec<usize> = gaps
        .iter()
        .enumerate()
        .filter(|&(_, x)| x == max_gap)
        .map(|(idx, _)| idx)
        .collect();

    // 若 max gaps >= 3，我们无论如何也没办法使得 max gap 缩小
    if max_gap_idxs.len() >= 3 {
        println!("{max_gap}");
        return;
    }

    // max{gaps[0] ~ gaps[i]};
    // [0..= (n - 2)]
    let pre_max: Vec<i64> = gaps
        .iter()
        .scan(0i64, |prev, x| {
            (*prev) = (*prev).max(*x);
            Some(*prev)
        })
        .collect();

    // max{gaps[i] ~ gaps[n - 1]}
    // [0..= (n - 2)]
    let suf_max: Vec<i64> = gaps
        .iter()
        .rev()
        .scan(0i64, |prev, x| {
            (*prev) = (*prev).max(*x);
            Some(*prev)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    // 评估翻转 [l, r] 产生的最大 gap
    let eval = |l: usize, r: usize, inner_max: i64| -> i64 {
        // 内部最大值翻倍
        let mut result = inner_max * 2;

        // 处理左侧
        if l > 0 {
            // 左侧未被影响部分
            if l > 1 {
                result = result.max(pre_max[l - 2]);
            }
            // 端点
            result = result.max((a[l - 1] - 2 * a[r]).abs());
        }

        // 处理右侧
        if r < n - 1 {
            // 右侧未被影响部分
            if r < n - 2 {
                result = result.max(suf_max[r + 1]);
            }
            // 端点
            result = result.max((a[r + 1] - 2 * a[l]).abs());
        }

        result
    };

    // 不进行任何操作
    let mut result = pre_max[n - 2];

    // 如果 gaps 有两个，则操作点一定在 [i + 1, j]
    if max_gap_idxs.len() == 2 {
        let l = max_gap_idxs[0] + 1;
        let r = max_gap_idxs[1];
        let mut inner_max = 0i64;
        for k in l..r {
            inner_max = inner_max.max(gaps[k]);
        }
        result = result.min(eval(l, r, inner_max));
    }

    // gaps 只有一个，需要枚举左右区间，[l, i], [i + 1, r]
    if max_gap_idxs.len() == 1 {
        let i = max_gap_idxs[0];

        let mut inner_max = 0i64;
        for l in (0..=i).rev() {
            if l < i {
                inner_max = inner_max.max(gaps[l]);
            }
            result = result.min(eval(l, i, inner_max));
        }

        let mut inner_max = 0i64;
        for r in i + 1..n {
            if r > i + 1 {
                inner_max = inner_max.max(gaps[r - 1]);
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
