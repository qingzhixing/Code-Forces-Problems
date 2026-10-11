#include <iostream>
#include <vector>
using namespace std;

const int MAX_N = 2e5 + 10;

void solution()
{
	int n;
	cin >> n;

	int x = 1;
	// 求最大的 x = 2 ^ k, 且 2 ^ k <= n
	while (2 * x <= n)
	{
		x *= 2;
	}

	// 前缀异或数组
	vector<int> prefix_xor;

	// 先放入 0 ~ x - 1
	for (int i = 0; i <= x - 1; i++)
	{
		prefix_xor.push_back(i);
	}
	for (int i = x - 1; i >= 0; i--)
	{
		prefix_xor.push_back(i);
	}

	// 再放入 x ~ 2 * x - 1
	for (int i = x; i <= 2 * x - 1; i++)
	{
		prefix_xor.push_back(i);
	}
	for (int i = 2 * x - 1; i >= x; i--)
	{
		prefix_xor.push_back(i);
	}

	// 还原原数组
	const auto len = prefix_xor.size() - 1;
	cout << len << endl;
	for (int i = 1; i <= len; i++)
	{
		// a[i] = prefix_xor[i] ^ prefix_xor[i - 1]
		cout << (prefix_xor[i] ^ prefix_xor[i - 1]) << ' ';
	}
	cout << endl;
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