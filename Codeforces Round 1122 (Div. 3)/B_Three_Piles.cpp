#include <iostream>
using namespace std;

void Solution()
{
	long long a, b, c;
	cin >> a >> b >> c;
	if (a >= b)
	{
		cout << a + c - b << endl;
		return;
	}
	// a < b
	if (abs(a - b) < abs(a + c - b))
	{
		cout << abs(a + c - b) << endl;
	}
	else
	{
		cout << abs(a - b) << endl;
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