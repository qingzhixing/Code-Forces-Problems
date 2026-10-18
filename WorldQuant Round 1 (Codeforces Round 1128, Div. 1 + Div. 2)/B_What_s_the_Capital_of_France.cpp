#include <iostream>
#include <vector>
using namespace std;

const int MAX_N = 2e5 + 10;
const int MAX_M = 1e9;
const int INF = MAX_M + 10;

void solution()
{
	int n;
	cin >> n;

	int max_num = -1, min_num = INF;
	for (int i = 1; i <= n; i++)
	{
		int a;
		cin >> a;
		max_num = max(max_num, a);
		min_num = min(min_num, a);
	}

	if (max_num == min_num)
	{
		cout << max_num << endl;
		return;
	}

	int result = 0;
	bool find_diff = false;
	for (int i = 31; i >= 0; i--)
	{
		if (find_diff)
		{
			result |= (1 << i);
			continue;
		}

		if (((max_num >> i) & 1) != ((min_num >> i) & 1))
		{
			result |= (1 << i);
			find_diff = true;
			continue;
		}

		result |= (max_num & (1 << i));
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