#include <iostream>
using namespace std;

const int MAX_N = 2e5 + 10;

void solution()
{
	int n, m;
	cin >> n >> m;

	if (n == 1)
	{
		cout << 0 << endl;
		return;
	}

	if (n == 2)
	{
		cout << 1 << endl
			 << "1 1" << endl;
		return;
	}

	if (n % 2 == 0)
	{
		n--;
	}

	// 用长度为 x 的区间叠加 x 次
	int x = (n + 1) / 2;

	cout << x << endl;
	for (int i = 1; i <= n - x + 1; i++)
	{
		cout << i << ' ' << i + x - 1 << endl;
	}
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