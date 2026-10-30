use std::collections::BTreeSet;
use std::io::{self, Read};
use std::str::{FromStr, SplitAsciiWhitespace};

const _MAX_N: usize = 0;
const _MAX_M: usize = 0;

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str(tokens: &mut SplitAsciiWhitespace) -> Vec<char> {
    tokens.next().unwrap().chars().collect()
}

fn reverse_pair_count(a: &mut [usize]) -> usize {
    let n = a.len();
    if n <= 1 {
        return 0;
    }

    // 辅助数组，长度与 a 相同；内容无所谓，只要长度够即可
    let mut tmp = a.to_vec();

    fn merge_sort(a: &mut [usize], tmp: &mut [usize]) -> usize {
        let n = a.len();
        if n <= 1 {
            return 0;
        }
        let mid = n / 2;

        // 递归处理左右两半
        let mut inv = merge_sort(&mut a[..mid], &mut tmp[..mid]);
        inv += merge_sort(&mut a[mid..], &mut tmp[mid..]);

        // 归并两个有序段，并统计跨段逆序对
        let (mut i, mut j, mut k) = (0, mid, 0);
        while i < mid && j < n {
            if a[i] <= a[j] {
                tmp[k] = a[i];
                i += 1;
            } else {
                tmp[k] = a[j];
                j += 1;
                // 左段 a[i..mid] 全部 > a[j]，与 a[j] 各构成一个逆序对
                inv += mid - i;
            }
            k += 1;
        }
        while i < mid {
            tmp[k] = a[i];
            i += 1;
            k += 1;
        }
        while j < n {
            tmp[k] = a[j];
            j += 1;
            k += 1;
        }

        // 写回原数组
        a.copy_from_slice(&tmp[..n]);
        inv
    }

    merge_sort(a, &mut tmp)
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let mut a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();
    let b: Vec<usize> = (0..n).map(|_| next(tokens)).collect();

    // 用 BTreeSet 维护 b
    let mut targets = BTreeSet::<(usize, usize)>::new();
    for i in 0..n {
        targets.insert((b[i], i));
    }

    // 贪心给每个 a[i] 分配一个最小的大于 a[i] 的 b[j]
    let mut ok = true;
    'outer: for i in 0..n {
        let key = (a[i], 0usize);
        if let Some(&(value, j)) = targets.range(key..).next() {
            a[i] = b[j];
            targets.remove(&(value, j));
        } else {
            // 无解
            ok = false;
            break 'outer;
        }
    }

    if !ok {
        println!("-1");
        return;
    }

    let result = reverse_pair_count(&mut a);
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
