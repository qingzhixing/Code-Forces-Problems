#include <iostream>
#include <map>
#include <vector>
#include <algorithm>
using namespace std;

const int MAX_N = 2e5 + 10;
const int MAX_M = 2e5 + 10;

void solution()
{
	int n, m;
	cin >> n >> m;

	vector<int> a(n + 1);
	// value, cnt
	map<int, int> cnt;

	for (int i = 1; i <= n; i++)
	{
		cin >> a[i];
		cnt[a[i]]++;
	}

	sort(a.begin() + 1, a.end());

	int result = 0;

	for (int number = 1; number <= m; number++)
	{
		// 对于当前数字, 只关心比它大的和它的两倍
		// 大于等于 number 的数字的数量
		const int greater_equal = a.end() - lower_bound(a.begin() + 1, a.end(), number);
		auto amount = greater_equal;
		if (cnt.find(number * 2) != cnt.end())
		{
			// number的两倍要额外贡献出一个
			amount += cnt[number * 2];
		}

		result = max(result, amount);
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