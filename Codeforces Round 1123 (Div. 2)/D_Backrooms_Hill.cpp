#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

#error TODO: Wrong Answer

const int MAX_N = 2e5 + 10;
const int MAX_M = 2e5 + 10;

int n;

void Solution()
{
	cin >> n;
	vector<int> odd, even;
	for (int i = 1; i <= n; i++)
	{
		int a;
		cin >> a;
		if (i % 2 == 1)
		{
			odd.push_back(a);
		}
		else
		{
			even.push_back(a);
		}
	}

	if (n == 1)
	{
		cout << "YES" << endl;
		return;
	}

	sort(odd.begin(), odd.end());
	sort(even.begin(), even.end());

	vector<int> after;

	// 直接合并，之后校验
	int idx = 0;
	while (idx < odd.size())
	{
		if (idx >= even.size())
		{
			after.push_back(odd[idx]);
			idx++;
			continue;
		}
		after.push_back(odd[idx]);
		after.push_back(even[idx]);

		idx++;
	}
	for (auto num : after)
	{
		cout << num << ' ';
	}
	cout << endl;

	// 判断线上升后下降
	bool up = true;
	int last_num = -1;
	for (auto num : after)
	{
		if (num > last_num)
		{
			if (!up)
			{
				// 可换向
				up = false;
			}
		}
		else
		// num < last_num
		{
			if (up)
			{
				cout << "NO" << endl;
				return;
			}
		}
		last_num = num;
	}
	cout << "YES" << endl;
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