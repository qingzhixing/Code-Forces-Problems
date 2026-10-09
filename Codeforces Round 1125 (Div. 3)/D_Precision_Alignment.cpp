#include <iostream>
#include <vector>
using namespace std;

const int MAX_N = 2e5 + 10;
const long long MAX_M = 1e18 + 10;

struct Laboratory
{
	long long value;
	// 想提升 value 需要先花费的转化操作次数
	long long prepayment;
	// 无法转化标记
	bool unmovable;
};

void solution()
{
	int n;
	long long k;
	cin >> n >> k;

	vector<Laboratory> labs;
	long long min_s = MAX_M;

	for (int i = 1; i <= n; i++)
	{
		long long a, b, c;
		cin >> a >> b >> c;

		const auto value = a + b + c;
		min_s = min(min_s, value);

		const bool unmovable = (a == b && b == c);

		auto prepayment = 0LL;

		if (a <= b && b <= c)
		{
			if (!unmovable)
			{
				// 减少 b 直到 b < a
				const auto trans_b = b - a + 1;
				// 减少 c 直到 c < b
				const auto trans_c = c - b + 1;
				const auto min_trans = min(trans_b, trans_c);
				prepayment = 2 * min_trans;
			}
		}

		labs.push_back({value, prepayment, unmovable});
	}

	auto feasible = [&](long long target)
	{
		auto total_cost = 0LL;
		for (const auto &[value, prepayment, unmovable] : labs)
		{
			// 如果已经达标则跳过
			if (value >= target)
			{
				continue;
			}

			// 未达标但是不可变则一定无法变成 target
			if (unmovable)
			{
				return false;
			}

			// 尝试变换
			const auto cost = target - value + prepayment;

			total_cost += cost;

			// 预先判断 k < total_cost
			if (k < total_cost)
			{
				return false;
			}
		}

		return k >= total_cost;
	};

	// 二分搜索 min_target
	auto l = min_s;
	auto r = min_s + k;
	while (l < r)
	{
		const auto mid = (l + r + 1) >> 1;
		if (feasible(mid))
		{
			// 当前点满足条件
			l = mid;
		}
		else
		{
			r = mid - 1;
		}
	}

	cout << l << endl;
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