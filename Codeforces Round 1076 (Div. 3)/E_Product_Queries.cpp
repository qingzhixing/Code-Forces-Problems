#include <iostream>
#include <vector>
#include <set>
#include <algorithm>
using namespace std;

const int MAX_N = 3e5 + 10;
const int IMPOSSIBLE = 114514191;

int n;
int dp[MAX_N];

void Solution()
{
	cin >> n;

	for (int i = 1; i <= n; i++)
	{
		dp[i] = IMPOSSIBLE;
	}

	for (int i = 1; i <= n; i++)
	{
		int num;
		cin >> num;
		dp[num] = 1;
	}

	for (int i = 1; i <= n; i++)
	{
		int times = 1;
		while (i * times <= n)
		{
			int target_number = i * times;
			dp[target_number] = min(dp[target_number], dp[i] + dp[times]);
			times++;
		}
	}

	for (int i = 1; i <= n; i++)
	{
		if (dp[i] == IMPOSSIBLE)
		{
			cout << -1 << " ";
		}
		else
		{
			cout << dp[i] << " ";
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
		Solution();
	}
	return 0;
}