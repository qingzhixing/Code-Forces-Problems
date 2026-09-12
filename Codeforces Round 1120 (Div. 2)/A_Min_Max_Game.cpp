#include <iostream>
using namespace std;

void Solution()
{
	int n;
	cin >> n;
	bool has_1 = false, has_0 = false;
	int cnt_0 = 0, cnt_1 = 0;
	for (int i = 1; i <= n; i++)
	{
		int num;
		cin >> num;
		if (num == 0)
		{
			cnt_0++;
			has_0 = true;
		}
		else
		{
			cnt_1++;
			has_1 = true;
		}
	}

	if (!has_0)
	{
		cout << "Bessie" << endl;
		return;
	}

	if (!has_1)
	{
		cout << "Elsie" << endl;
		return;
	}

	if (cnt_1 >= cnt_0)
	{
		cout << "Bessie" << endl;
		return;
	}

	cout << "Elsie" << endl;
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