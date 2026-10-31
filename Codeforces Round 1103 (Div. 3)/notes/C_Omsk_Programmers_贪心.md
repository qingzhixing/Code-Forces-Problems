# C. Omsk Programmers

## 题目信息

题目链接：[codeforces.com/contest/2236/problem/C](https://codeforces.com/contest/2236/problem/C)

Clist Rating: 932

算法标签：暴力枚举 贪心 数学

## 题目大意

给定三个数字 a b x，进行任意多次操作，希望 a 变成 b。求最小操作数。

操作方式：

1. 将 a or b +=1
2. a or b = $\lfloor \frac{a\ or\ b}{x} \rfloor$

## 算法思路

```Rust
fn solution(tokens: &mut SplitAsciiWhitespace) {
    let mut a: i64 = next(tokens);
    let mut b: i64 = next(tokens);
    let x: i64 = next(tokens);

    let mut result = (a - b).abs();
    let mut div_time = 0;
    while a != b {
        div_time += 1;
        if a > b {
            a /= x;
        } else {
            b /= x;
        }
        result = result.min(div_time + (a - b).abs());
    }

    println!("{result}");
}
```

## 复杂度

时间复杂度：O(log n)

空间复杂度：O(1)
