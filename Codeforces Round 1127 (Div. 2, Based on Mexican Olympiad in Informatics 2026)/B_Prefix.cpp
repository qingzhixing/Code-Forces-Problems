#include <iostream>
#include <climits>
using namespace std;

const int MAX_N = 60;
const int MAX_M = 50;

void solution()
{
	int n;
	cin >> n;
	long long max_a = LLONG_MIN;

	// 当前 s 的上下界
	long long L = 0, H = 0;

	for (int i = 1; i <= n; i++)
	{
		long long a;
		cin >> a;

		max_a = max(max_a, a + max(abs(L), abs(H)));

		L = min(a, a + 2 * L);
		H = max(a, a + 2 * H);
	}

	cout << max_a << endl;
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