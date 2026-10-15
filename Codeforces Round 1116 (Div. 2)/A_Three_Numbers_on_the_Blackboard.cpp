#include <iostream>
using namespace std;

void solution()
{
	int a, b, c;
	cin >> a >> b >> c;
	int low = min(a, min(b, c));
	int high = max(a, max(b, c));
	int mid = a + b + c - low - high;
	cout << min(high - low, mid) << endl;
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