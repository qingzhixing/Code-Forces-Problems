#include <iostream>
#include <cstring>
using namespace std;

static const int MAX_N = 2e5 + 10;

int n, k;
bool exist[MAX_N];

void Solution()
{
	memset(exist, 0, sizeof(exist));
	cin >> n >> k;
	int lack = 0, much = 0;
	int value;
	for (int i = 1; i <= n; i++)
	{
		cin >> value;
		if (value > k)
		{
			continue;
		}
		if (value == k)
		{
			much++;
			continue;
		}
		if (value < k)
		{
			exist[value] = true;
			continue;
		}
	}

	// 判断空缺
	for (int i = 0; i <= k - 1; i++)
	{
		if (!exist[i])
			lack++;
	}

	cout << max(lack, much) << endl;
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