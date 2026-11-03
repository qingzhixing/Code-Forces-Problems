# B. Another Sorting Problem

## 题目信息

Clist Rating: 1040

算法标签：构造

## 题目大意

给定一个正整数数组 a，对 a 中任意多元素进行操作，让他们的值增加任意正整数值。判断 能不能通过至多一次操作使得 a 变为非降序？

## 关键观察

- 题目要求非递减，那么可以转化为相邻项中后一个值大于等于前一个值
- 所有选择的数字需要共用一个 k ，那么 k 一定需要大到能够修复最大的相邻项差值

## 算法思路

```Rust
fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let mut a: Vec<i64> = (0..n).map(|_| next(tokens)).collect();

    let max_diff = a.windows(2).map(|w| w[0] - w[1]).max().unwrap().max(0);
    for i in 0..n {
        if i != 0 && a[i] < a[i - 1] {
            a[i] += max_diff;
        }
    }

    for i in 0..n {
        if i != 0 && a[i] < a[i - 1] {
            println!("NO");
            return;
        }
    }
    println!("YES");
}
```

## 触发点

- 非递减 -> 后面的值 >= 前一个
- 最多操作一次 + 选择共同的数 -> 这个数一定要满足最坏情况
