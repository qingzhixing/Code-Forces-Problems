#include <iostream>
#include <vector>
#include <bitset>
using namespace std;

const int MAX_N = 2e5 + 10;

// 记录每个数字的质因数
vector<int> factors[MAX_N];

void init_factors()
{
	bitset<MAX_N> is_prime;
	is_prime.set();

	is_prime[0] = false;
	is_prime[1] = false;

	for (int p = 2; p < MAX_N; p++)
	{
		if (!is_prime[p])
		{
			continue;
		}

		// 将当前数字加入它倍数的质因子列表中
		for (int multiple = p; multiple < MAX_N; multiple += p)
		{
			factors[multiple].push_back(p);
			// 筛掉 p 的倍数
			if (multiple > p)
			{
				is_prime[multiple] = false;
			}
		}
	}
}

void Solution()
{
	int n, k;
	cin >> n >> k;

	vector<int> a(n);
	for (int i = 0; i < n; i++)
	{
		cin >> a[i];
	}

	// dp[i] 表示将 i 变为所有元素都 <= k 的最小操作数
	vector<long long> dp(n + 1, 0x3f3f3f3f3f3f3f3f);
	for (int i = 1; i <= n; i++)
	{
		if (i <= k)
		{
			dp[i] = 0;
			continue;
		}

		for (const auto &p : factors[i])
		{
			dp[i] = min(dp[i], 1LL + 1LL * p * dp[i / p]);
		}
	}

	long long result = 0;
	for (const auto &number : a)
	{
		result += dp[number];
	}
	cout << result << endl;
}

int main()
{
	init_factors();
	int t;
	cin >> t;
	while (t--)
	{
		Solution();
	}
	return 0;
}