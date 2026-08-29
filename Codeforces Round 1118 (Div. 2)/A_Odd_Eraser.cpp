#include <iostream>
using namespace std;

int gcd(int a, int b)
{
	if (b == 0)
	{
		return a;
	}
	return gcd(b, a % b);
}

int main()
{
	int t;
	cin >> t;
	while (t--)
	{
		int n;
		cin >> n;
		int a, b;
		int num;
		for (int i = 1; i <= n; i++)
		{
			cin >> num;
			if (i == 1)
			{
				a = num;
			}
			if (i == n)
			{
				b = num;
			}
		}
		cout << gcd(a, b) << endl;
	}
	return 0;
}