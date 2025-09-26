#include <iostream>
using namespace std;

static const int MAX_N = (2e5 + 10);

int main()
{
	int t;
	cin >> t;
	while (t--)
	{
		int n, k, array[MAX_N]{};
		// cnt[i] 用于统计数值为 i 的数字在数组中出现了几次
		int cnt[MAX_N]{};

		cin >> n >> k;

		for (auto index = 1; index <= n; index++)
		{
			cin >> array[index];
			cnt[array[index]]++;
		}

		// 判断非法情况
		if (n % k)
		{
			cout << 0 << endl;
			continue;
		}
		bool illegal = false;
		for (auto index = 1; index <= n; index++)
		{
			if (cnt[array[index]] % k)
			{
				cout << 0 << endl;
				illegal = true;
				break;
			}
		}
		if (illegal)
			continue;

		// range_cnt[i] 用于统计数值为 i 的数字在当前区间中出现了几次
		int range_cnt[MAX_N]{};
		long long ans{};

		// 双指针判断,对于确定的右端点找到最小的左端点 l
		for (int l = 1, r = 1; r >= l && r <= n; r++)
		{
			range_cnt[array[r]]++;

			// 如果超过了当前数字区间最大容纳量则移动左指针直到符合要求
			while (range_cnt[array[r]] > (cnt[array[r]] / k))
			{
				range_cnt[array[l]]--;
				l++;
			}

			// 所有子数组 a[r,r],a[r-1,r],…,a[l,r] 都满足条件
			ans += (r - l + 1);
		}

		cout << ans << endl;
	}
	return 0;
}