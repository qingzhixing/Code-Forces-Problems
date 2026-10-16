#include <iostream>
#include <vector>
using namespace std;

const int MAX_N = 2e5 + 10;

// 代价不超过 cost 时能容纳的最多橘子数
int get_capacity(int cost)
{
	int result = 0;
	for (int i = 1; i <= cost; i++)
	{
		result += i * (cost / i);
	}
	return result;
}

void solution()
{
	int n;
	cin >> n;

	// 二分搜索答案
	int l = 0, r = 510;
	while (l < r)
	{
		const int mid = (l + r) >> 1;
		const int capacity = get_capacity(mid);
		if (capacity >= n)
		{
			r = mid;
		}
		else
		{
			l = mid + 1;
		}
	}

	// 找到最小代价，尝试构造答案
	const int cost = l;
	const int capacity = get_capacity(cost);

	vector<int> bags;
	for (int i = 1; i <= cost; i++)
	{
		// 数量为 i 的袋子的数量
		const int cnt = cost / i;
		for (int j = 1; j <= cnt; j++)
		{
			bags.push_back(i);
		}
	}

	// bags 现在已经是从小到大排好序的
	int remove = capacity - n;

	// 当前正在移除哪个袋子中的橘子
	int idx = 0;
	while (remove > 0 && idx < bags.size())
	{
		if (bags[idx] == 0)
		{
			idx++;
			continue;
		}
		if (remove >= bags[idx])
		{
			remove -= bags[idx];
			bags[idx] = 0;
			continue;
		}
		bags[idx] -= remove;
		remove = 0;
	}

	// 删除空袋子
	vector<int> final_bags;
	for (const auto &item : bags)
	{
		if (item > 0)
		{
			final_bags.push_back(item);
		}
	}

	// 输出答案
	cout << cost << endl;
	for (int bag_id = 0; bag_id < final_bags.size(); bag_id++)
	{
		const auto &cnt = final_bags[bag_id];
		for (int i = 1; i <= cnt; i++)
		{
			cout << bag_id + 1 << ' ';
		}
	}
	cout << endl;
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