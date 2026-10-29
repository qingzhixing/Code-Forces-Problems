# C. Subarrays

## 题目信息

题目链接：[https://codeforces.com/contest/2276/problem/C](https://codeforces.com/contest/2276/problem/C%E2%80%B8)

Clist Rating：1088

算法标签：数论 数学

## 题目大意

给定一个长度为 n 的正整数数组 a , 求 a 中长度 > k, 且元素和 % k == 0 的子数组个数

## 我的思路

- 要求子数组元素和 %k == 0，我们可以转化为 前缀和同余。即对于区间 `[l, r]`，其中元素和 % k == 0 可以转化为 `p[l - 1] === p[r] (mod k)`

## 卡点

- 我不知道如何处理出 与当前节点距离为 k 的合法左端点个数

## 算法思路

- 我们可以使用滑动窗口来解决这个问题
- 不断枚举右端点 `r`，右端点向右移动时，可以新加入考量的只有一点 `l = r - k - 1`，我们每次统计不断加入区间的左端点即可维护一个严格与右端点距离 > k 的左端点集合。

```Rust
let mut result = 0usize;
// cnt[i] 距离 r 距离 > k 的 模为 i 的 前缀和数量
let mut cnt = vec![0usize; k + 1];
// 不断向右枚举 区间右端点
for r in k + 1..=n {
    // 新进入该滑动窗口的左端点
    let new_l = r - k - 1;
    cnt[presum_mod[new_l]] += 1;
    // 当前右端点能产生的贡献
    result += cnt[presum_mod[r]];
}
```

## 复杂度

时间复杂度：O(n)

空间复杂度：O(n)

## 触发点

- 遇到维护某个区间长度 >= k 时，可以采用滑动窗口的思路，仅有满足条件的点能进入区间。
