#include <iostream>
using namespace std;

int main()
{
	int t;
	cin >> t;

	int n, m;
	while (t--)
	{
		cin >> n >> m;
		int current_time = 0;
		bool current_side = 0;
		int ans = 0;
		while (n--)
		{
			int quest_time, side;
			cin >> quest_time >> side;

			int delta_time = quest_time - current_time;
			/*
			 * 在同一侧，ans + 时间差向偶向下对齐
			 * 不在同一侧，ans + 时间差向奇向下对齐
			 */
			ans += delta_time;
			if (current_side == side)
			{
				ans -= delta_time & 1;
			}
			else
			{
				ans -= !(delta_time & 1);
			}

			current_time = quest_time;
			current_side = side;
		}

		ans += m - current_time;

		cout << ans << endl;
	}
	return 0;
}