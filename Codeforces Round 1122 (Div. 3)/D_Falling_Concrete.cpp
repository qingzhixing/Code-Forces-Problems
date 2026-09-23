#include <iostream>
#include <set>
using namespace std;

const int MAX_N = 2e5 + 10;

void Solution()
{
	int n;
	cin >> n;

	set<int> values;
	for (int i = 1; i <= n; i++)
	{
		int a;
		cin >> a;
		values.insert(a - i);
	}

	int result = 0;
	// 当前段的长度
	int current_length = 0;
	// 上一个值
	int last = -1;

	for (auto x : values)
	{
		// 当前段断开，结算上一段
		if (last != -1 && x != (last + 1))
		{
			result = max(result, current_length);
			current_length = 0;
		}

		// 将当前值加入当前段
		current_length++;

		last = x;
	}

	// 最后一段
	result = max(result, current_length);

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