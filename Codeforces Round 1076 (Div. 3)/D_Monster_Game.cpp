#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

void Solution()
{
	int n;
	cin >> n;
	vector<long long> a(n + 10), b(n + 10);
	for (auto i = 1; i <= n; i++)
	{
		cin >> a[i];
	}

	sort(a.begin() + 1, a.begin() + 1 + n);

	for (auto i = 1; i <= n; i++)
	{
		cin >> b[i];
	}

	vector<long long> pre_sum_b(n + 10);
	// 计算 b 的前缀和数组
	for (auto i = 1; i <= n; i++)
	{
		pre_sum_b[i] = pre_sum_b[i - 1] + b[i];
	}

	long long score = 0;

	// 用于记录上一个难度通过的level数，用来优化时间
	long long last_max_level = n;

	long long last_difficulty = -1, last_index = -1;
	for (int i = 1; i <= n; i++)
	{
		// 更新难度
		if (last_difficulty == -1)
		{
			last_difficulty = a[i];
			last_index = i;
		}
		else if (a[i] != last_difficulty)
		{
			last_difficulty = a[i];
			last_index = i;
		}
		else
		{
			continue;
		}

		// 计算当前难度下剑的个数
		int sword_cnt = n - last_index + 1;

		int max_level = 0;

		for (int lvl_idx = last_max_level; lvl_idx >= 1; lvl_idx--)
		{
			if (sword_cnt >= pre_sum_b[lvl_idx])
			{
				max_level = lvl_idx;
				break;
			}
		}

		score =
			max(score,
				max_level * last_difficulty);

		last_max_level = max_level;
	}

	cout << score << endl;
}

int main()
{
	int t;
	cin >> t;
	while (t--)
	{
		Solution();
	}
	return 0;
}