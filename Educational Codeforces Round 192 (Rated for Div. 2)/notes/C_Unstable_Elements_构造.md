# C. Unstable Elements

## 题目信息 (Problem Info)

* **Clist Rating**: 1260
* **算法标签**：游程编码（连续段缩点）、操作不变量分析、枚举存活集合、同余与整除判定

## 一句话题意 (One-Sentence Summary)

给定一个长度为 $n$ 的非降序整数数组 $a$，每次操作将所有段首元素（首元素及所有满足 $a_i \neq a_{i-1}$ 的元素）标记，并选择将其**全部删除**或**全部复制一份**（数组为空时不可继续操作），求经过任意次操作后能得到的长度为 $k$ 的不同数组个数。

## 关键观察/转化 (Key Observations/Transformations)

* **观察一：有序性保证连续段永不合并，操作等价于全体存活段长度同步 $\pm 1$**
* 将有序数组 $a$ 按极长连续相等元素划分为 $m$ 个段，对应数值严格递增为 $v_1 < v_2 < \dots < v_m$，初始长度依次为 $L_1, L_2, \dots, L_m$。
* **不合并证明**：由于原数组单调不降，相同数值最初必处于同一个极长连续段中。即便后续某些中间段因长度归零而被彻底删除，剩余存活段的数值序列 $v_{i_1} < v_{i_2} < \dots < v_{i_r}$ 依然**严格递增**，相邻存活段数值永远不等，绝不会发生两段合并为一段的情况。
* 因此，任意非空时刻被标记的元素**恒为当前每个存活段的唯一段首**：执行一次“删除”等价于所有存活段长度同时 $-1$（长度为 $1$ 的段归零并永久消失）；执行一次“复制”等价于所有存活段长度同时 $+1$（不产生新段，也不复活已消失的段）。
* **观察二：存活段集合仅由“历史最大净删除次数”决定**
* **状态刻画证明**：设任意合法操作序列中，相对于初始状态曾达到过的**最大净删除次数**为 $D$（由于空数组不可再操作，必有 $0 \le D < \max_i L_i$）。
* 若某段初始长度 $L_i \le D$，则在首次达到净删除 $D$ 次时（或更早）其长度已降至 $0$ 并永久消失；
* 若某段初始长度 $L_i > D$，则在整个过程中其长度始终 $\ge L_i - D \ge 1$，全程存活。
* 由此可知，最终存活的段集合必为按初始长度截断的后缀集合 $S = \{i \mid L_i \ge \text{len}\}$，其中 $\text{len}$ 取遍原数组中实际出现的不同段长（至多 $\mathcal{O}(\sqrt{n})$ 种）。
* **观察三：固定存活集合下的可达长度充要条件与唯一性**
* 设当前枚举的最短存活段长为 $\text{len}$，对应存活集合 $S = \{i \mid L_i \ge \text{len}\}$ 的段数为 $\text{rest\_type} = \vert{}S\vert{}$，初始总长度为 $\text{rest\_num} = \sum_{i \in S} L_i$。令该集合能达到的最小总长度为：

$$
\text{min\_len} = \text{rest\_num} - (\text{len} - 1) \times \text{rest\_type}

$$

则集合 $S$ 能拼出长度 $k$ 的充要条件为：

$$
k \ge \text{min\_len} \quad \text{且} \quad (k - \text{min\_len}) \bmod \text{rest\_type} = 0

$$

* **必要性证明**：要使所有 $L_i \ge \text{len}$ 的段全部存活，历史最大净删除次数至多为 $\text{len} - 1$，故最终相对于初始状态的净变化量 $\Delta$（存活段最终长度 $L'_i = L_i + \Delta$）必须满足 $\Delta \ge -(\text{len} - 1)$。令非负整数 $c = \Delta + (\text{len} - 1) \ge 0$，则最终总长度必满足：

$$
k = \sum_{i \in S} (L_i + \Delta) = \text{rest\_num} - (\text{len} - 1) \cdot \text{rest\_type} + c \cdot \text{rest\_type} = \text{min\_len} + c \cdot \text{rest\_type}

$$

因此必有 $k \ge \text{min\_len}$ 且 $k - \text{min\_len}$ 是 $\text{rest\_type}$ 的非负整数倍。

* **充分性（构造性）证明**：设不同段长排序后为 $l_1 < l_2 < \dots < l_U$，当前 $\text{len} = l_j$。若条件成立，取 $c = \frac{k - \text{min\_len}}{\text{rest\_type}} \ge 0$：
* 先连续执行 $\text{len} - 1$ 次**删除**：由于前一档段长（若 $j > 1$）满足 $l_{j-1} \le l_j - 1 = \text{len} - 1$，所有满足 $L_p < \text{len}$ 的段全部归零淘汰；而所有 $i \in S$ 的段剩余长度为 $L_i - (\text{len} - 1) \ge 1$，此时数组总长度为 $\text{min\_len} \ge \text{rest\_type} \ge 1$（非空合法）。
* 再连续执行 $c$ 次**复制**：每个存活段长度各加 $c$，总长度恰好变为 $\text{min\_len} + c \cdot \text{rest\_type} = k$。
* **不重不漏（双射唯一性）证明**：
* 不同 $\text{len}$ 对应的存活集合 $S$ 所包含的**元素数值种类集合** $\{v_i \mid i \in S\}$ 严格不同（元素种类数严格递减），故不同轮次生成的最终数组绝不重复；
* 同一存活集合 $S$ 内，给定总长度 $k$ 后，净变化量 $\Delta = \frac{k - \text{rest\_num}}{\text{rest\_type}}$ 唯一确定，从而每一段的最终长度 $L_i + \Delta$ 唯一确定，因此每个合法集合对答案的贡献严格为 $1$。

## 算法步骤 (Algorithm Steps)

* **游程编码与段长分桶**：扫描有序数组 $a$，统计极长连续相同元素段的长度，并用 `BTreeMap`（`len_freq`）记录每个段长 `len` 对应的段数 `cnt`。
* **初始化存活总量**：设存活段的初始元素总和 `rest_num = n`，存活段总数 `rest_type = len_freq.values().sum()`，可达方案数 `result = 0`。
* **枚举最小存活段长并判定可达性**：按 `len` 从小到大遍历 `len_freq`，计算保住当前所有存活段的最小总长度 `min_len = rest_num - (len - 1) * rest_type`；若满足 `k >= min_len && (k - min_len) % rest_type == 0`，则将 `result` 加 $1$。
* **淘汰当前最短段**：判定完毕后，将原长为 `len` 的 `cnt` 个段从存活集合中剔除，更新 `rest_num -= len * cnt` 与 `rest_type -= cnt`，继续下一轮循环；遍历结束后输出 `result`。

## 复杂度 (Complexity)

* **时间复杂度**：$\mathcal{O}(n)$（精确为 $\mathcal{O}(n + U \log U)$，其中 $U$ 为不同连续段长度的种类数；由于 $\sum_{j=1}^U l_j \le n$，必有 $U = \mathcal{O}(\sqrt{n})$，故 $U \log U = o(n)$，多测总复杂度为 $\mathcal{O}(\sum n)$）。
* **空间复杂度**：$\mathcal{O}(n)$（存储输入数组占 $\mathcal{O}(n)$，`BTreeMap` 至多存储 $U = \mathcal{O}(\sqrt{n})$ 个键值对）。

## 触发信号 (Trigger Signals)

* **有序序列**中针对所有满足 $a_i \neq a_{i-1}$ 的位置（各连续段段首）同步执行删除/复制操作 $\implies$ 转化为**极长连续段长度全局同步 $\pm 1$**，且利用原序列有序性可直接得出**中间段消亡后相邻存活段永不合并**的不变量。
* 全局同步加减操作中带有“减到 $0$ 即永久消亡”的吸收边界 $\implies$ 存活子集仅由**历史最深删除水位线**决定，按初始段长排序后**从小到大枚举存活后缀集合**。
* 固定存活子集后单次操作对总长度的改变量恒为存活段数 $C$ $\implies$ 算出最深合法状态总长 `min_len`，用 **`k >= min_len` 配合模 $C$ 同余**在 $\mathcal{O}(1)$ 内判定可达性。
