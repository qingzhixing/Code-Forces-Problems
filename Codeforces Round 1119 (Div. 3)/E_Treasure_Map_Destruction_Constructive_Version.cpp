#include <iostream>
#include <cmath>
#include <cstring>
using namespace std;

const int MAX_N = 2e5 + 10;

int t;
int n;
int a[MAX_N];
int prefix_sum[MAX_N];

int main()
{
	cin >> t;
	while (t--)
	{
		memset(prefix_sum, 0, sizeof(prefix_sum));
		cin >> n;

		for (int i = 1; i <= n; i++)
		{
			cin >> a[i];
			if (a[i] >= 1)
			{
				int left = max(1, i - a[i] + 1);
				int right = min(n, i + a[i] - 1);
				prefix_sum[left] += 1;
				prefix_sum[right + 1] += -1;
			}
		}

		// 求前缀和
		// prefix_sum[i] != 0 说明该位置不能放置宝藏
		for (int i = 1; i <= n; i++)
		{
			prefix_sum[i] += prefix_sum[i - 1];
		}

		// 枚举所有数字看看是否其满足条件
		bool no_ans = false;
		for (int i = 1; i <= n; i++)
		{
			// 非零数字满足左右必定有宝藏
			if (a[i] > 0)
			{
				int left = i - a[i];
				int right = i + a[i];
				bool left_treasure = false;
				bool right_treasure = false;
				if (left >= 1 && prefix_sum[left] == 0)
				{
					left_treasure = true;
				}
				if (right <= n && prefix_sum[right] == 0)
				{
					right_treasure = true;
				}
				if (!left_treasure && !right_treasure)
				{
					no_ans = true;
					break;
				}
			}
			// 0 满足其一定不在标记点上
			else if (a[i] == 0)
			{
				if (prefix_sum[i] != 0)
				{
					no_ans = true;
					break;
				}
			}
		}

		if (no_ans)
		{
			cout << -1 << endl;
			continue;
		}

		for (int i = 1; i <= n; i++)
		{
			cout << (prefix_sum[i] == 0 ? 1 : 0);
		}
		cout << endl;
	}
	return 0;
}