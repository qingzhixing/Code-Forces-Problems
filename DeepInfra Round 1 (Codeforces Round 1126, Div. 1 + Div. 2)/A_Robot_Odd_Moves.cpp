#include <iostream>
using namespace std;

void solution()
{
	int a, b;
	cin >> a >> b;
	if (a < 0 || abs(b) > a + 1)
	{
		cout << -1 << endl;
		return;
	}
	b = abs(b);
	if ((a - b) & 1)
	{
		cout << a + 1 << endl;
	}
	else
	{
		cout << a << endl;
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