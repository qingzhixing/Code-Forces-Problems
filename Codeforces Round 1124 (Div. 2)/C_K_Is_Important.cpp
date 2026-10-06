#include <iostream>
using namespace std;

const int MAX_N = 1e5 + 10;

int n, k;
int a[MAX_N];

void Solution()
{
	cin >> n >> k;
	for (int i = 1; i <= n; i++)
	{
		cin >> a[i];
	}

	long long result = 0;

	int l, r;

	if (n >= 2 * k)
	{
		// a[k] ~ a[n - k + 1] 可以全部选择
		for (int i = k; i <= n - k + 1; i++)
		{
			result += a[i];
		}

		l = k - 1;
		r = n - k + 1 + 1;
	}
	else
	{
		l = n - k + 1;
		r = k;
	}

	// 余下部分选大的
	while (l >= 1 && r <= n)
	{
		result += max(a[l], a[r]);
		++r;
		--l;
	}

	cout << result << endl;
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