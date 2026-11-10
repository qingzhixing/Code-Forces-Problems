# B. Knife's Pill Farm

## 题目信息 (Problem Info)

B. Knife's Pill Farm
算法标签：数学，贪心，优先队列（大根堆）

## 一句话题意 (One-Sentence Summary)

从给定的数组中按原相对顺序挑选 $m$ 个数组成子序列 $b$，使得带权得分公式 $\sum_{i=1}^{m} i \cdot (b_i - b_{i-1})$ 的值最大。

## 关键观察/转化 (Key Observations/Transformations)

**公式化简**是本题破局的绝对核心。将要求的总得分公式展开：
$\sum_{i=1}^{m} i \cdot (b_i - b_{i-1})$
$= (1 \cdot b_1 + 2 \cdot b_2 + \dots + m \cdot b_m) - (1 \cdot b_0 + 2 \cdot b_1 + 3 \cdot b_2 + \dots + m \cdot b_{m-1})$
因为 $b_0 = 0$，上下两式相加合并同类项后，除了 $b_m$ 的系数是 $m$ 以外，所有 $b_i$ ($1 \le i \le m-1$) 的系数都是 $-1$。
因此目标函数化简为：$m \cdot b_m - \sum_{i=1}^{m-1} b_i$。
这个优美的转化意味着：前 $m-1$ 个数的相对顺序根本不影响得分，我们只需要枚举 $b_m$，并在它之前的元素中贪心地选出**最小的 $m-1$ 个数**即可。

## 算法步骤 (Algorithm Steps)

1. 从左到右遍历数组 $a$。
2. 使用一个大根堆（BinaryHeap）来动态维护遍历过程中最小的 $m-1$ 个元素，并用 `heap_sum` 实时记录这 $m-1$ 个元素的和。
3. 当堆的大小不足 $m-1$ 时，直接将当前元素入堆。
4. 当堆的大小等于 $m-1$ 时，说明已经有足够的前置元素。此时可以将当前遍历到的 $a_i$ 视作 $b_m$，计算当前组合的得分 `a[i] * m - heap_sum`，并更新全局最大值 `result`。
5. 尝试将当前的 $a_i$ 加入堆中：如果它比堆顶（即前 $m-1$ 小元素中的最大值）还要小，则弹出堆顶并将 $a_i$ 入堆，同步更新 `heap_sum`，为后续枚举 $b_m$ 提供更优的前提条件。

## 复杂度 (Complexity)

* **Time Complexity**: $\mathcal{O}(n \log m)$。只需遍历一次长度为 $n$ 的数组，每次堆的插入和弹出操作耗时 $\mathcal{O}(\log m)$。
* **Space Complexity**: $\mathcal{O}(m)$。大根堆的大小被严格限制在 $m-1$。

## 触发信号 (Trigger Signals)

* 看到公式中存在 $\sum i \cdot (b_i - b_{i-1})$ 这种前后项差值乘以递增系数的形式 $\to$ 毫不犹豫地拆开求和符号并合并同类项，通常会消去大量中间变量，只剩下首尾和常数系数。
* 需要在一个不断向右扩张的数组前缀中，动态维护前 $k$ 小（或前 $k$ 大）元素的和 $\to$ 固定使用大小为 $k$ 的大根堆（或小根堆）。
