#include <iostream>
using namespace std;

const int MAX_N = 1e6 + 10;
const int MAX_M = 1e6;

void solution()
{
	int n;
	cin >> n;
	int even = 0, odd = 0;
	while (n--)
	{
		int a;
		cin >> a;
		if (a % 2 == 1)
		{
			odd++;
		}
		else
		{
			even++;
		}
	}
	if ((odd == 1 && even == 1) || (odd == 3 && even == 0))
	{
		cout << "Larpity" << endl;
	}
	else
	{
		cout << "Moggity" << endl;
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