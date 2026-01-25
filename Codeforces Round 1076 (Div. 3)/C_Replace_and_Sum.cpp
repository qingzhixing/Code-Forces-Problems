#include <iostream>
#include <vector>
using namespace std;

void Solution()
{
	int n, q;
	cin >> n >> q;
	vector<int> a(n + 10), b(n + 10), result(n + 10);

	for (auto i = 1; i <= n; i++)
	{
		cin >> a[i];
	}
	for (auto i = 1; i <= n; i++)
	{
		cin >> b[i];
	}
	for (auto i = n; i >= 1; i--)
	{
		result[i] =
			max(
				max(a[i], b[i]),
				result[i + 1]);
	}

	// 建立前缀和
	vector<int> prefix_sum(n + 10);
	for (auto i = 1; i <= n; i++)
	{
		prefix_sum[i] = prefix_sum[i - 1] + result[i];
	}

	// query
	while (q--)
	{
		int l, r;
		cin >> l >> r;
		cout << prefix_sum[r] - prefix_sum[l - 1] << ' ';
	}

	cout << endl;
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