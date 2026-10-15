#include <iostream>
using namespace std;

void solution()
{
	int a, b;
	cin >> a >> b;

	if (a >= 2 * b)
	{
		cout << b << endl;
		return;
	}

	if (a <= b)
	{
		cout << -1 << endl;
		return;
	}

	int result = 0;
	while (a > b + 1)
	{
		a -= 2;
		b--;
		result++;
	}
	result++;
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