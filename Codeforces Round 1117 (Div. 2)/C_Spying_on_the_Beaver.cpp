#include <iostream>
#include <vector>
#include <set>
using namespace std;

const int MAX_N = 1e5 + 10;
const int MAX_M = MAX_N;

void solution()
{
	int n, m;
	cin >> n;

	vector<int> p(n + 1);

	for (int i = 2; i <= n; i++)
	{
		cin >> p[i];
	}

	cin >> m;

	set<int> dams;
	for (int i = 1; i <= m; i++)
	{
		int a;
		cin >> a;
		dams.insert(a);
	}

	cout << m - 1 << ' ';

	// 忽略 dams 第一项(有 root 则一定是 root)
	bool skip = true;
	for (const auto id : dams)
	{
		if (skip)
		{
			skip = false;
			continue;
		}
		cout << id << ' ';
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