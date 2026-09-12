#include <iostream>
#include <vector>
using namespace std;

const int MAX_N = 1010;

void Solution()
{
	int n, k;
	cin >> n >> k;
	if (k < n || k >= 2 * n)
	{
		cout << -1 << endl;
		return;
	}

	vector<vector<int>> a(n + 10, vector<int>(n + 10, 0));
	// 计算需要重合的数量
	int same = 2 * n - k;
	// 当前填充到的数字
	int current = 1;

	// 填充 (1, 1) 到 (same, same) 对角线
	for (int i = 1; i <= same; i++)
	{
		a[i][i] = current;
		current++;
	}

	// 填充 (same, same + 1) to (same, n)
	for (int i = same + 1; i <= n; i++)
	{
		a[same][i] = current;
		current++;
	}

	// 填充 (same + 1, same) to (n, same)
	for (int i = same + 1; i <= n; i++)
	{
		a[i][same] = current;
		current++;
	}

	// 填充剩余
	for (int i = 1; i <= n; i++)
	{
		for (int j = 1; j <= n; j++)
		{
			if (a[i][j] == 0)
			{
				a[i][j] = current;
				current++;
			}
		}
	}

	// 输出
	for (int i = 1; i <= n; i++)
	{
		for (int j = 1; j <= n; j++)
		{
			cout << a[i][j] << ' ';
		}
		cout << endl;
	}
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