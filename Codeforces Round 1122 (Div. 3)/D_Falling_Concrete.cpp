#include <iostream>
using namespace std;

#error TODO: Uncomplemented. Wrong Answer.

const int MAX_N = 2e5 + 10;

int n;
int a[MAX_N];

void Solution()
{
	cin >> n;
	for (int i = 1; i <= n; i++)
	{
		cin >> a[i];
	}

	int result = 0;

	int last_num = 0;
	int same_cnt = 0;
	for (int i = 1; i <= n; i++)
	{
		if (a[i] == last_num)
		{
			same_cnt++;
		}
		else
		{
			same_cnt = 1;
		}
		result = max(result, same_cnt);
		last_num = a[i];
	}

	bool operated = true;
	while (operated)
	{

		operated = false;
		for (int i = 2; i <= n; i++)
		{
			if (a[i] - a[i - 1] >= 2)
			{
				operated = true;
				a[i]--;
				a[i - 1]++;
				swap(a[i], a[i - 1]);
				break;
			}
		}

		int last_num = 0;
		int same_cnt = 0;
		for (int i = 1; i <= n; i++)
		{
			if (a[i] == last_num)
			{
				same_cnt++;
			}
			else
			{
				same_cnt = 1;
			}
			result = max(result, same_cnt);
			last_num = a[i];
		}
	}

	cout << result << endl;
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