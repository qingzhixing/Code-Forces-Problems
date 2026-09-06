#include <iostream>
#include <cstring>
#include <unordered_map>
using namespace std;

const int MAX_N = 2e5 + 10;

int t;
int n;
int a[MAX_N];
int appear[MAX_N];
int k;

int main()
{
	cin >> t;
	while (t--)
	{

		cin >> n;

		memset(appear, 0, (n+10) * sizeof(int));

		unordered_map<int, int> num_map;
		for (int i = 1; i <= n; i++)
		{
			cin >> a[i];
			num_map[a[i]]++;
		}
		// 判断解的情况
		auto zero_num = num_map[0];

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

		// 构造两个集合 mex = k, 另一个 mex = 0
		// get max k
		for (int i = 0;; i++)
		{
			if (num_map[i] < 2)
			{
				k = i;
				break;
			}
		}

		cout << "Yes" << endl;
		for (int i = 1; i <= n; i++)
		{
			if (a[i] >= k)
			{
				cout << 'C';
				continue;
			}
			if (appear[a[i]])
			{
				cout << 'B';
				continue;
			}
			appear[a[i]] = true;
			cout << 'A';
		}
		cout << endl;
	}
	return 0;
}