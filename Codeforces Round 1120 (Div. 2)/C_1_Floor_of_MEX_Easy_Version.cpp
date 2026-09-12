#include <iostream>
#include <vector>
using namespace std;

const int MAX_N = 1e5 + 10;

void Solution()
{
	int n;
	cin >> n;

	// 之后对 flag 求前缀和，求完之后的值为 0 表示它在 B 中
	vector<int> flag(n + 10, 0);

	for (int i = 1; i <= n; i++)
	{
		int a;
		cin >> a;
		long long left = max(0LL, 1LL * a * i);
		long long right = min(n - 1LL, max(1LL * (a + 1) * i - 1, 0LL));
		// 没法排除任何数字
		if (left >= n)
		{
			continue;
		}
		flag[left]++;
		flag[right + 1]--;
	}

	// 求前缀和
	for (int i = 1; i <= n - 1; i++)
	{
		flag[i] += flag[i - 1];
	}

	vector<int> b;
	// 得到解
	for (int i = 0; i < n; i++)
	{
		if (flag[i] == 0)
		{
			b.push_back(i);
		}
	}

	cout << b.size() << endl;
	for (auto num : b)
	{
		cout << num << ' ';
	}
	cout << endl;
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