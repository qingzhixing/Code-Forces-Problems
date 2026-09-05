#include <iostream>
using namespace std;

const int MAX_N = 2e5 + 10;

int t;
int n;
int a[MAX_N];

int main()
{
	cin >> t;
	while (t--)
	{
		cin >> n;
		for (int i = 1; i <= n; i++)
		{
			cin >> a[i];
		}

		for (int i = 1; i <= n; i++)
		{
			if (a[i] == 1)
			{
				break;
			}
			if (a[i] == -1)
			{
				a[i] = 1;
				break;
			}
		}

		for (int i = n; i >= 1; i--)
		{
			if (a[i] == 1)
			{
				break;
			}
			if (a[i] == -1)
			{
				a[i] = 1;
				break;
			}
		}

		for (int i = 1; i <= n; i++)
		{
			if (a[i] == -1)
			{
				a[i] = 0;
			}
			cout << a[i] << ' ';
		}
		cout << endl;
	}
	return 0;
}