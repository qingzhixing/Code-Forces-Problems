#include <iostream>
using namespace std;

void solution()
{
	int x, y;
	cin >> x >> y;

	const auto max_xor = x + y;
	// 将 x 转化为 max_xor 中的二进制 1 凑出的最大不大于 x 的数字 的转化次数
	auto rest = x;

	// 从高往低贪心枚举
	for (auto digit = 30; digit >= 0; digit--)
	{
		if ((max_xor >> digit) & 1)
		{
			if (rest >= (1 << digit))
			{
				rest -= (1 << digit);
			}
		}
	}

	printf("%d %d\n", max_xor, rest);
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