#include <iostream>
#include <vector>
using namespace std;

const int MAX_N = 110;
const int MAX_M = 1e9;

void solution()
{
	int n, m;
	cin >> n >> m;

	vector<int> a(n + 10);
	vector<int> b(m + 10);

	long long sum_a = 0, sum_b = 0;

	for (int i = 1; i <= n; i++)
	{
		cin >> a[i];
	}

	for (int i = 1; i <= m; i++)
	{
		cin >> b[i];
	}

	sum_a += a[n];
	for (int i = 1; i < n; i++)
	{
		sum_a += (a[i] - a[i + 1]) + 1;
	}

	sum_b += b[m];
	for (int i = 1; i < m; i++)
	{
		sum_b += (b[i] - b[i + 1]) + 1;
	}

	// printf("sum_a = %d, sum_b = %d\n", sum_a, sum_b);

	if (sum_a >= sum_b)
	{
		cout << 1 << endl;
	}
	else
	{
		cout << 2 << endl;
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