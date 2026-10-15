#include <iostream>
#include <vector>
using namespace std;

const int MAX_N = 2e5 + 10;
const int MAX_M = 1e9;

void solution()
{
	int n, k;
	cin >> n >> k;

	// a[i] 前缀和 mod k
	vector<long long> p(n + 10);

	for (int i = 1; i <= n; i++)
	{
		int a;
		cin >> a;
		p[i] = (p[i - 1] + a) % k;
	}

	long long result = 0;
	// cnt[i] = p[j] mod k 余数为 i 的 p[j] 个数
	vector<long long> cnt(k);
	for (int r = k + 1; r <= n; r++)
	{
		// 新纳入考量的 p[l]
		const int new_l = r - k - 1;
		cnt[p[new_l]]++;
		result += cnt[p[r]];
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