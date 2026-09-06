#include <iostream>
#include <cstring>
#include <unordered_map>
using namespace std;

const int MAX_N = 2e5 + 10;

int t;
int n;
int a[MAX_N];
int appear[MAX_N];

int main()
{
	cin >> t;
	while (t--)
	{

		cin >> n;

		memset(appear, 0, (n + 10) * sizeof(int));

		auto zero_num = 0;
		for (int i = 1; i <= n; i++)
		{
			cin >> a[i];

			if (a[i] == 0)
			{
				zero_num++;
			}
		}

		// 判断解的情况
		// 所有集合 mex = 0
		if (zero_num == 0)
		{
			cout << "Yes" << endl;
			for (int i = 1; i <= n; i++)
			{
				cout << 'A';
			}
			cout << endl;
			continue;
		}

		// 两个集合 mex = 0, 另一个非零, 无解
		if (zero_num == 1)
		{
			cout << "No" << endl;
			continue;
		}

		// 构造两个集合 mex = 1, 另一个mex = 0

		cout << "Yes" << endl;
		bool zero_A = false;
		for (int i = 1; i <= n; i++)
		{
			if (a[i] == 0)
			{
				if (!zero_A)
				{
					zero_A = true;
					cout << 'A';
				}
				else
				{
					cout << 'B';
				}
			}
			else
			{
				cout << 'C';
			}
		}
		cout << endl;
	}
	return 0;
}