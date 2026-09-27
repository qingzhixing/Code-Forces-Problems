#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

const int MAX_N = 2e5 + 10;

int n;
// 记录每个数字所在位置的奇偶
bool odd_position[MAX_N];

void Solution()
{
	cin >> n;
	for (int i = 1; i <= n; i++)
	{
		int number;
		cin >> number;
		odd_position[number] = i % 2;
	}

	// 对于一个合格的序列，其任意后缀都满足
	// 奇数坐标数量 和 偶数坐标数量 的差的绝对值 <= 1
	// 奇数坐标数量 - 偶数坐标数量
	int delta = 0;
	for (int i = n; i >= 1; i--)
	{
		if (odd_position[i])
		{
			delta++;
		}
		else
		{
			delta--;
		}

		if (abs(delta) > 1)
		{
			cout << "NO" << endl;
			return;
		}
	}

	cout << "YES" << endl;
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