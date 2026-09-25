#include <iostream>
#include <cstring>
using namespace std;

const int MAX_N = 110;
const int MAX_M = 110;

int n;
int a[MAX_M];

void Solution()
{
	memset(a, 0, sizeof(a));
	cin >> n;
	for (int i = 1; i <= n; i++)
	{
		int num;
		cin >> num;
		a[num]++;
	}
	for (int i = 1; i <= n; i++)
	{
		for (int j = 100; j >= 1; j--)
		{
			if (a[j] != 0)
			{
				a[j]--;
				cout << j << ' ';
			}
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