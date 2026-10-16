#include <iostream>
#include <vector>
#include <bitset>
using namespace std;

const int MAX_N = 2e5 + 10;
const int MAX_M = MAX_N;

void solution()
{
	int n;
	cin >> n;

	vector<int> a(n + 10);
	int result = 0;
	for (int i = 1; i <= n; i++)
	{
		cin >> a[i];
		if (a[i] != a[i - 1])
		{
			result++;
		}
	}

	vector<bool> delete_tag(n + 10);
	for (int i = 1; i <= n; i++)
	{
		if (a[i] == a[i - 1] || a[i] == a[i + 1])
		{
			delete_tag[i] = true;
		}
	}

	// 交换后新产生的数字的数量
	int delta = 0;
	for (int i = 1; i <= n; i++)
	{
		if (!delete_tag[i])
		{
			continue;
		}

		// 尝试向左交换
		if (i > 1)
		{
			if (delete_tag[i - 1] && a[i - 1] != a[i])
			{
				delta = max(delta, 2);
			}
			if (!delete_tag[i - 1] && ((i > 2 && a[i - 2] != a[i]) || i <= 2))
			{
				delta = max(delta, 1);
			}
		}

		// 尝试向右交换
		if (i < n)
		{
			if (delete_tag[i + 1] && a[i + 1] != a[i])
			{
				delta = max(delta, 2);
			}
			if (!delete_tag[i + 1] && ((i < n - 1 && a[i + 2] != a[i]) || i >= n - 1))
			{
				delta = max(delta, 1);
			}
		}
	}

	// printf("result:%d, delta:%d\n", result, delta);
	cout << result + delta << endl;
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