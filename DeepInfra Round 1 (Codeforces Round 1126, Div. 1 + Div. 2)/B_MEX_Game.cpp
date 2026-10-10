#include <iostream>
#include <vector>
#include <map>
using namespace std;

const int MAX_N = 2e5 + 10;
const int MAX_M = 2e5;
const int INF = MAX_M + 10;

void solution()
{
	int n, k;
	cin >> n >> k;

	vector<int> a(n);
	map<int, int> counter;
	for (auto &item : a)
	{
		cin >> item;
		counter[item]++;
	}

	int mex_a_max = INF, mex_b_max = INF;
	for (const auto &[number, cnt] : counter)
	{
		if (cnt < 2 * k - 1)
		{
			mex_a_max = min(mex_a_max, number);
			mex_b_max = min(mex_b_max, number);
			continue;
		}
		if (cnt < 2 * k)
		{
			mex_b_max = min(mex_b_max, number);
		}
	}

	if (mex_a_max != mex_b_max)
	{
		cout << "YES" << endl;
	}
	else
	{
		cout << "NO" << endl;
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