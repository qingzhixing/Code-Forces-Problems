#include <iostream>
using namespace std;

const long long MAX_N = 1e6 + 10;
const long long MAX_M = 1e12;

void solution()
{
	long long x, y, k;
	cin >> x >> y >> k;

	if (y == x)
	{
		cout << 0 << endl;
		return;
	}

	if (y < x)
	{
		cout << (2 * y + k - 1) * k / 2 << endl;
		return;
	}

	auto quotient = y / x;
	auto remainder = y % x;
	auto result = 0LL;
	while (k)
	{
		if (quotient == 1)
		{
			result += k * remainder;
			k = 0;
			continue;
		}
		// 下一个除数
		x++;
		result += remainder;
		remainder -= (quotient - 1);
		while (remainder < 0)
		{
			remainder += x;
			quotient--;
		}

		// 步进 1
		k--;
	}
	cout << result << endl;
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