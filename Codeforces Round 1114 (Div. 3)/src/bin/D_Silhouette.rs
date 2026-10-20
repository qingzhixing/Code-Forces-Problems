use std::collections::BTreeMap;
use std::io::{self, Read, Write};
use std::str::{FromStr, SplitAsciiWhitespace};

const _MAX_N: usize = 2e5 as usize;
const _MAX_M: usize = 2e14 as usize;

fn next<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    tokens.next().unwrap().parse().ok().unwrap()
}

#[allow(dead_code)]
fn next_str<'a>(tokens: &mut SplitAsciiWhitespace<'a>) -> &'a str {
    tokens.next().unwrap()
}

fn solution(tokens: &mut SplitAsciiWhitespace, out: &mut impl Write) {
    let n: usize = next(tokens);
    let a: Vec<i64> = (0..n).map(|_| next(tokens)).collect();

    if !a.contains(&0) {
        writeln!(out, "-1").unwrap();
        return;
    }

    let mut freq: BTreeMap<i64, i64> = BTreeMap::new();
    let mut ans: BTreeMap<i64, i64> = BTreeMap::new();

    for &number in &a {
        *freq.entry(number).or_insert(0) += 1;
    }

    if freq.len() == 1 {
        for _ in 0..n {
            write!(out, "1 ").unwrap();
        }
        writeln!(out).unwrap();
        return;
    }

    let entries: Vec<_> = freq.into_iter().collect();
    // 前面项数字的和
    let mut pre_sum = 0i64;
    for idx in 0..entries.len() {
        let (number, cnt) = entries[idx];

        if idx == entries.len() - 1 {
            // 最后一个数为前一个数 + 1
            let (pre_number, _) = entries[idx - 1];

            ans.insert(number, ans[&pre_number] + 1);
            continue;
        }
        let (nxt_number, _) = entries[idx + 1];

        // 计算当前数字的答案
        if nxt_number <= pre_sum || (nxt_number - pre_sum) % cnt != 0 {
            writeln!(out, "-1").unwrap();
            return;
        }

        let current_ans = (nxt_number - pre_sum) / cnt;
        ans.insert(number, current_ans);
        pre_sum += current_ans * cnt;

        // 判断是否比前一个大
        if idx > 0 {
            let (pre_number, _) = entries[idx - 1];
            if ans[&pre_number] >= current_ans {
                writeln!(out, "-1").unwrap();
                return;
            }
        }
    }

    for number in a {
        write!(out, "{} ", ans[&number]).unwrap();
    }
    writeln!(out).unwrap();
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut out = io::BufWriter::new(io::stdout().lock());

    let t: usize = next(&mut tokens);
    for _ in 0..t {
        solution(&mut tokens, &mut out);
    }
    out.flush().unwrap();
}
