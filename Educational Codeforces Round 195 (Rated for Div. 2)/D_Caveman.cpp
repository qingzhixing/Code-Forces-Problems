#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

const int MAX_N = 3e5 + 10;
const int MAX_M = 3e5 + 10;

void solution()
{
	int n, m;
	cin >> n >> m;

	vector<int> degree(n);
	while (m--)
	{
		int u, v;
		cin >> u >> v;

		// 0 - indexed
		--u;
		--v;

		degree[u]++;
		degree[v]++;
	}

	sort(degree.begin(), degree.end());

	// 存在孤立点
	if (degree[0] == 0)
	{
		cout << "Brain Melt" << endl;
		return;
	}

	// 星图
	if (degree[n - 2] == 1 && degree[n - 1] == n - 1)
	{
		cout << 2 << endl;
		return;
	}

	// 其他则解多于 2 个
	cout << "Many" << endl;
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