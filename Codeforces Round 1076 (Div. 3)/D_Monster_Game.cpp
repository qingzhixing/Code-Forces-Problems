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

	long long score = 0;

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

		// cout << "+ Current Difficulty: " << last_difficulty << endl;
		// cout << "+ Current Index: " << last_index << endl;

		// 计算当前难度下剑的个数
		int sword_cnt = n - last_index + 1;

		// cout << "+ Sword Count: " << sword_cnt << endl;

		int passed_level = 0;

		// 尝试通关
		while (passed_level < n && sword_cnt >= b.at(passed_level + 1))
		{
			// cout << "Passed Level: " << level + 1 << endl;
			// cout << "Before sword count: " << sword_cnt << endl;
			// cout << "Used sword count: " << b.at(level + 1) << endl;

			sword_cnt -= b.at(passed_level + 1);
			passed_level++;

			// cout << "Last sword:" << sword_cnt << endl;
		}
		// cout << "+ Achieved Level: " << level << endl;

		score =
			max(score,
				passed_level * last_difficulty);

		// cout << "+ Max Score: " << score << endl;
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