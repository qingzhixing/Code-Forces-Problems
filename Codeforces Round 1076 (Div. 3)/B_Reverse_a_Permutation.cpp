#include <iostream>
#include <cstring>
using namespace std;

const int MAX_N = 2e5 + 10;

int n, p[MAX_N];

void Solution()
{
	memset(p, 0, sizeof(p));
	cin >> n;
	int start = -1, end = -1;
	for (auto i = 0; i < n; i++)
	{
		cin >> p[i];
		if (start == -1)
		{
			if (p[i] != n - i)
			{
				start = i;
			}
		}
		else
		{
			if (p[i] == n - start)
			{
				end = i;
			}
		}
	}

	if ((start != -1) && (end != -1))
	{
		for (auto i = 0; i < start; i++)
		{
			cout << p[i] << ' ';
		}
		for (auto i = end; i >= start; i--)
		{
			cout << p[i] << ' ';
		}
		for (auto i = end + 1; i < n; i++)
		{
			cout << p[i] << ' ';
		}
	}
	else
	{
		for (auto i = 0; i < n; i++)
		{
			cout << p[i] << ' ';
		}
	}

	cout << endl;
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