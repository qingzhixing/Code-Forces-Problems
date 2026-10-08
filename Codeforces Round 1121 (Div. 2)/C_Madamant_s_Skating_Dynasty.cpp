#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

const int MAX_N = 2e5 + 10;
const int MAX_M = 1e9 + 10;
const int MOD = 998244353;

void solution()
{
	int n;
	cin >> n;

	vector<long long> a(n + 1);
	for (int i = 1; i <= n; i++)
	{
		cin >> a[i];
	}

	sort(a.begin() + 1, a.end());

	// suffix_sum[i] = a[i] + ... + a[n]
	vector<long long> suffix_sum(n + 2);
	for (int i = n; i >= 1; i--)
	{
		suffix_sum[i] = (suffix_sum[i + 1] + a[i]) % MOD;
	}

	// prefix_ways[i] = product_{k = 1}^{i - 1} (n - k)
	// 表示 1 ~ i (i 节点此时无父节点) 节点自由选择父亲的方式数
	vector<long long> prefix_ways(n + 1);
	prefix_ways[1] = 1;
	for (int i = 2; i <= n; i++)
	{
		prefix_ways[i] = prefix_ways[i - 1] * (n - (i - 1)) % MOD;
	}

	// prefix_ways[i] = product_{k = i}^{n - 1} (n - k)
	// 表示 i ~ n (n 节点此时无父节点) 节点自由选择父亲的方式数
	vector<long long> suffix_ways(n + 2);
	suffix_ways[n] = 1;
	for (int i = n - 1; i >= 1; i--)
	{
		suffix_ways[i] = suffix_ways[i + 1] * (n - i) % MOD;
	}

	long long result = 0;
	// 固定一个节点 i 枚举其所有的父亲 j，统计 i -> j 总共贡献的代价
	for (int i = 1; i <= n - 1; i++)
	{
		// i -> j 的边的总代价
		// i 可以选择父亲 j = i + 1 ... n，共 n-i 个
		// 所有可能父亲带来的总成本：
		// sum_{j = i + 1}^{n} (b[j] - b[i])
		long long cost = (suffix_sum[i + 1] - (n - i) * a[i]) % MOD;
		cost = (cost + MOD) % MOD;

		// 每条边出现的次数
		// 固定 i 的父亲后，其他孩子 k != i 仍然自由选择父亲
		// 每个 k 有 n-k 种选择
		// ways = product_{k = 1, k != i}^{n - 1} (n - k)
		long long ways = prefix_ways[i] * suffix_ways[i + 1] % MOD;

		result = (result + (cost * ways % MOD)) % MOD;
	}

	cout << result << endl;
}

int main()
{
	int t;
	cin >> t;
	while (t--)
	{
		solution();
	}
	return 0;
}