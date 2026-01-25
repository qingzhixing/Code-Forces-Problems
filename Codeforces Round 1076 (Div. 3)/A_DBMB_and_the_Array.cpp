#include <iostream>
using namespace std;

void Solution()
{
	int n, s, x;
	cin >> n >> s >> x;
	int element, sum = 0;
	for (int i = 1; i <= n; i++)
	{
		cin >> element;
		sum += element;
	}
	int delta = s - sum;

	if (delta < 0)
	{
		cout << "NO" << endl;
		return;
	}

	if (delta == 0)
	{
		cout << "YES" << endl;
		return;
	}

	if (delta > 0)
	{
		cout << ((delta % x) ? "NO" : "YES") << endl;
		return;
	}
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