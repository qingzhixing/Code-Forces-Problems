# B. MEX Game

## 题目信息

题目链接：[codeforces.com/contest/2271/problem/B](https://codeforces.com/contest/2271/problem/B)

Clist Rating: 920

算法标签：博弈论

## 题目大意

一共 $n (n \in \mathbb{N}^* \land 2 \mid n)$ 张牌，每张牌数值为 $a_i \in [0, n]$，给定一个 $k \in \mathbb{N}^*$;

设 $\operatorname{mex}_k(\mathcal{S})$ 是出现在 $\mathcal{S}$ 中且出现次数少于 $k$ 次的最小非负整数;

有 A 和 B 两人轮流拿取牌堆中的牌，若$\operatorname{mex}_k(\mathcal{A}) \ne \operatorname{mex}_k(\mathcal{B})$ 则 A 赢，否则 B 赢。

二者都用最优策略，判断 A 是否能赢？

## 关键观察

令 $\operatorname{freq}(x)$ 表示 $x$ 在牌堆中的频率；

若 $\operatorname{freq}(num) \ge 2k$ 那么 B 一定能模仿 A一样的操作让这个数字对 $\operatorname{mex}$ 相等无影响；

若 $\operatorname{freq}(num) = 2k - 1$ 那么 A 总能取到 $k$ 个 num 但是 B 不行，这样这个数就会对 $\operatorname{mex}$ 相等性有影响；

若 $\operatorname{freq}(num) < 2k - 1$ 那么只要 B 模仿 A 操作，就可以让当前数字对 $\operatorname{mex}$ 相等无影响，并且二者的 $\operatorname{mex}$ 最大值被确定；

## 算法思路

```C++
// 找到第一个 freq[x] < 2k 的 x
int x = 0;
while (x <= n && freq[x] >= 2 * k) x++;

// 判定：如果这个位置恰好是 2k-1，Alice 必胜
if (x <= n && freq[x] == 2 * k - 1) {
    cout << "YES\n";
} else {
    cout << "NO\n";
}
```

### 为什么循环在第一个 `freq[x] < 2k` 处停止就够了？

题目要求的是 $\operatorname{mex}_k(\mathcal{A})\ne \operatorname{mex}_k(\mathcal{B})$。

如果双方 $\operatorname{mex}_k$ 相等，设公共值为 $y$，则：

- 对所有 $v<y$，双方都至少有 $k$ 张 $v$，所以这些数的总数必须 $\ge 2k$；
- 对 $v=y$，双方都少于 $k$ 张，所以 $freq[y]\le 2k-2$。

因此，**公共 mex 只能出现在第一个 $freq[x]<2k$ 的位置**。在它之前的所有数，总数都至少 $2k$，Bob 都能通过模仿 / 配对策略让双方都拿到至少 $k$ 张，不会成为分歧点。

所以扫到第一个 $freq[x]<2k$ 就可以停：后面的数再怎么样，也无法改变 $x$ 这里已经注定的结局。

## 复杂度

时间复杂度：$O(n)$

空间复杂度：$O(n)$

## 触发点

- 偶数的 n, 轮流拿，各拿一半，且是**零和判断“是否相等”** 可以考虑 **配对 / 镜像策略**
- **mex 是“最小”的 → 只需看第一个出问题的数**
