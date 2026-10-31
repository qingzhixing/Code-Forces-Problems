# D. Precision Alignment

## 题目信息

题目链接：[codeforces.com/contest/2275/problem/D](https://codeforces.com/contest/2275/problem/D)

Clist Rating: 1370

算法标签：二分 暴力枚举 贪心 模拟

## 题目大意

有 $n$ 个实验室，每个实验室有三个整数读数 $a_i,b_i,c_i$，顺序固定。

每次操作可以选择一个实验室，并执行下面三种调整之一：

- $c_i \gets c_i + \operatorname{sgn}(a_i-b_i)$
- $b_i \gets b_i + \operatorname{sgn}(a_i-c_i)$
- $a_i \gets a_i + \operatorname{sgn}(b_i-c_i)$

其中 $\operatorname{sgn}(x)$ 在 $x>0,=0,<0$ 时分别为 $1,0,-1$。

也就是说，每次操作会看另外两个读数的差，决定把某个读数加 $1$、减 $1$ 或不变。

你最多可以做 $k$ 次操作，可以作用在任意实验室上，也可以重复操作同一个实验室。

最终，每个实验室的评分是

$$
a_i+b_i+c_i

$$

而总体评价取所有实验室评分的最小值：

$$
S=\min_i (a_i+b_i+c_i)

$$

要求最大化这个最小值 $S$。

## 关键观察

- 每次有效操作最多让一个实验室的总和增加 $1$；
- 如果当前实验室处于“好状态”，可以不断让总和增加；
- 如果处于坏状态，例如 $a\le b\le c$，需要先花费一些操作进入好状态；
- 如果 $a=b=c$，则三个读数完全相等，任何操作都不会改变它们，总和无法增加；

## 算法思路

这是一道典型的二分题。

可以对最终目标 $S$ 二分答案，检查每个实验室达到至少 $S$ 所需的最少操作数之和是否不超过 $k$。

```Rust
let check = |goal: i64| -> bool {
    let mut remain = k;
    for &Laboratory {
        movable,
        prepayment,
        rating,
    } in &labs
    {
        if rating >= goal {
            continue;
        }
        if !movable {
            return false;
        }
        let needed = prepayment + goal - rating;
        if remain < needed {
            return false;
        }
        remain -= needed;
    }
    true
};
```

## 复杂度

时间复杂度：$O(n)$

空间复杂度：$O(n)$

## 触发点

- 问“最大化最小值” → 二分答案
