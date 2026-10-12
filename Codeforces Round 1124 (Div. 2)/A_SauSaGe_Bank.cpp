#include <iostream>
using namespace std;

void solution()
{
	int n, k;
	cin >> n >> k;
	cout << (k - 1) * 2 + (1 << (n - k + 1)) << endl;
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