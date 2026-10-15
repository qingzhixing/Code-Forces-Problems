#include <iostream>
#include <vector>
using namespace std;

const int MAX_N = 1e5 + 10;
const int MAX_M = 1e4;

void solution()
{
	int n;
	cin >> n;
	vector<long long> a(n + 1);
	for (int i = 1; i <= n; i++)
	{
		cin >> a[i];
	}

	long long p = 0;
	for (int i = n; i >= 1; i--)
	{
		p -= 2 * (p - a[i]);
	}

	cout << p / 2 << endl;
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