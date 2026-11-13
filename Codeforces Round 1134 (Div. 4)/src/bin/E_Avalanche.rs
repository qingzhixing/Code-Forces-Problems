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

/// 外部接口：0-indexed，区间采用左闭右开 [l, r)
pub struct XorFenwick {
    size: usize,
    // 内部 1-indexed，维护异或差分数组
    tree: Vec<u8>,
}

#[inline]
fn lowbit(num: usize) -> usize {
    num & num.wrapping_neg()
}

#[allow(dead_code)]
impl XorFenwick {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            tree: vec![0u8; size + 1],
        }
    }

    /// 直接从 0 / 1 奇偶数组 进行 O(n) 线性建树
    pub fn from_slice(array: &[u8]) -> Self {
        let size = array.len();
        let mut tree = vec![0u8; size + 1];
        let mut prev = 0u8;

        for (i, &val) in array.iter().enumerate() {
            let cur = val & 1; // 确保只取最低位 0/1
            let idx = i + 1;

            tree[idx] ^= cur ^ prev;
            prev = cur;

            let parent = idx + lowbit(idx);
            if parent <= size {
                tree[parent] ^= tree[idx];
            }
        }

        Self { size, tree }
    }

    /// 修改 0-indexed 差分数组的单点, 翻转原数组后缀 [idx, size)
    fn flip_diff(&mut self, idx: usize) {
        // to 1 - indexed
        let mut idx = idx + 1;
        while idx <= self.size {
            self.tree[idx] ^= 1;
            // to parent
            idx += lowbit(idx);
        }
    }

    /// 0 - indexed 区间奇偶翻转：左闭右开 [l, r)
    pub fn range_flip(&mut self, l: usize, r: usize) {
        debug_assert!(l <= r && r <= self.size);
        if l == r {
            return;
        }
        self.flip_diff(l);
        self.flip_diff(r);
    }

    /// 0 - indexed 原数组单点奇偶翻转
    pub fn point_flip(&mut self, idx: usize) {
        debug_assert!(idx < self.size);
        self.range_flip(idx, idx + 1);
    }

    /// 0 - indexed 查询当前点的奇偶性 0 even 1 odd
    pub fn query(&self, current: usize) -> u8 {
        debug_assert!(current < self.size);
        let mut idx = current + 1;
        let mut sum = 0u8;
        while idx > 0 {
            sum ^= self.tree[idx];
            idx -= lowbit(idx); // 跳转到前一个前缀管辖区间
        }
        sum
    }
}

fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let k: usize = next(tokens);
    let a: Vec<u8> = (0..n).map(|_| (next::<i64>(tokens) % 2) as u8).collect();
    let mut fenwick = XorFenwick::from_slice(&a);
    let mut b_win = true;
    // 从后往前遍历 a 奇偶性
    for i in (0..n).rev() {
        // 偶数点不影响结果
        if fenwick.query(i) == 0 {
            continue;
        }
        // 奇数点影响结果，并且会改变前面最多 k 个数的奇偶性
        b_win = !b_win;
        let start = 0.max(i as i64 - k as i64) as usize;
        let end = i;
        fenwick.range_flip(start, end);
    }
    if b_win {
        println!("Bob");
    } else {
        println!("Alice");
    }
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
