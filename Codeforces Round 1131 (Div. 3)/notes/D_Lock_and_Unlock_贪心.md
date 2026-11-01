# D. Lock and Unlock

## 题目信息

题目链接：[codeforces.com/contest/2278/problem/D](https://codeforces.com/contest/2278/problem/D)

Clist Rating: 1178

算法标签：贪心

## 题目大意

给定数组 $a$，初始全部未锁定。每次操作：

1. 选择 $k$ 个不同下标锁定，花费 $k$；
2. 让所有**未锁定**元素整体 $+1$ 或整体 $-1$；
3. 全部解锁。

目标：让所有元素相等。先最小化总花费，再在最小花费前提下最小化操作次数。

## 关键观察

- 题目是任意选定下标，所以答案与原数组顺序无关，可以进行排序。
- 设最终所有元素都变成 x，x 可以是任意整数，不一定在原数组中。
- 要求所有数字相等，等价于消除所有相邻数字的差。

## 算法思路

### 最小化代价

设排序后的数组为：

$$
b_1\le b_2\le \cdots \le b_n

$$

要将 b[i], b[i+1] 之间的距离缩短 1, 假设我们选择增加 b[i]，如果不一起联动前缀，那么缩小了 b[i], b[i+1] 之间的间隔之后同时也会扩大 b[i-1], b[i] 之间的间隔。所以为了不逆转以前的操作并且不增加新的间隔，我们需要一次性移动某个 前缀 或者 后缀。

所以想要让 b[i+1] - b[i] 减小一个单位，我们有如下两种最优操作：

- 锁定 b[1] ~ b[i] (cost = i)，并减少整个后缀 b[i+1] ~ b[n];
- 锁定 b[i+1] ~ b[n] (cost = n - i)，并增加整个前缀 b[1] ~ b[i];

所以缩小一次这个窗口的最小代价为 min(i, n - i)

### 最小化操作数

按上述方式进行操作，最少需要 $b_n-b_1$ 次操作。因为我们要把最大的差变为 0。

```Rust
fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let mut a: Vec<i64> = (0..n).map(|_| next(tokens)).collect();
    a.sort_unstable();

    let ops = a[n - 1] - a[0];
    let mut cost = 0usize;
    for i in 0..n - 1 {
        cost += (a[i + 1] - a[i]) as usize * (i + 1).min(n - 1 - i);
    }
    println!("{cost} {ops}");
}
```

## 复杂度

时间复杂度：O(n log n)

空间复杂度：O(n)

## 触发点

- 可以将让所有数字相等转化为消除相邻元素之间的差。
