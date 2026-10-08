#include <iostream>
#include <queue>
#include <vector>
#include <climits>
using namespace std;

const int MAX_N = 0;
const int MAX_M = 0;

void solution()
{
	int n, m;

	cin >> n >> m;

	vector<long long> a(n + 1);

	long long max_a = LLONG_MIN;

	for (int i = 1; i <= n; i++)
	{
		cin >> a[i];
		max_a = max(max_a, a[i]);
	}

	// 特判
	if (m == 1)
	{
		cout << max_a << endl;
		return;
	}

	// 维护一个 m - 1 大小的大根堆
	priority_queue<long long> q;
	// 前 m - 1 个元素的相反数和
	long long queue_sum = 0;
	long long result = LLONG_MIN;
	for (int i = 1; i <= n; i++)
	{
		if (i >= m)
		{
			result = max(result, queue_sum + m * a[i]);
		}

		// 将当前数字加入维护堆中
		if (q.size() < m - 1)
		{
			q.push(a[i]);
			queue_sum -= a[i];
		}
		else if (a[i] < q.top())
		{
			const auto deleted = q.top();
			q.pop();
			q.push(a[i]);
			queue_sum += deleted;
			queue_sum -= a[i];
		}
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