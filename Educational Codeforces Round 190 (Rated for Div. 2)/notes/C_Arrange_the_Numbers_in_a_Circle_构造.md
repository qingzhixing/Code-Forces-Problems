# C. Arrange the Numbers in a Circle

## 题目信息

Clist Rating: 1262

算法标签：构造 模拟 数学

## 题目大意

给定 若干张写有 1 ~ n  的牌，值为 i 的牌有 ci 张。ci 满足非下降。我们从中选取至少三张未成一圈，要求任意大小为 3 的窗口中，一定要至少有两张相同数字的牌。问最多可以选多少张。

## 关键观察

选取不同数字的所有牌，如果他们所有都 >= 2 张，那么相同数字放一块，总是满足要求的。

## 算法思路

- ci 非下降，我们可以从后往前枚举 >= 2 数量的牌，他们总是能放在一起，可以加入答案。
- 剩下的全是只有一张的牌。我们模拟一下可以观察到，如果某一个数字的牌 >= 4 ，将产生一个位置可以多容纳一张单独的牌。它会把这个数字的牌堆分为两部分，每个部分都 >=2 张，窗口中心为单独牌时，所有相等，所以这个操作合法。对于 >= 4 张的牌，我们总能将其分为 c/2 向下取整 堆 >= 2 的堆，每堆之间可以容纳一张牌，一共能容纳 c/2向下取整 - 1 张牌。
- 特别的，如果只有一堆 >= 2 的牌堆，它将能多产生一个容纳位，因为它首尾相连。

```Rust
fn solution(tokens: &mut SplitAsciiWhitespace) {
    let n: usize = next(tokens);
    let a: Vec<usize> = (0..n).map(|_| next(tokens)).collect();
    let mut result = 0usize;
    // 能容纳单独一张牌的数量
    let mut capacity = 0usize;
    let mut not_1_cnt = 0usize;
    for num in a.into_iter().rev() {
        if num > 1 {
            result += num;
            not_1_cnt += 1;
            capacity += (num / 2) - 1;
        } else {
            if not_1_cnt == 1 {
                capacity += 1;
                not_1_cnt = 0;
            }
            if capacity > 0 {
                capacity -= 1;
                result += 1;
            }
        }
    }

    if result <= 2 {
        println!("0");
    } else {
        println!("{result}");
    }
}
```
