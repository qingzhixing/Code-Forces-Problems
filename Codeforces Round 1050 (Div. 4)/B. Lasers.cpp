#include <iostream>
using namespace std;

int main()
{
	int t;
	cin >> t;

	int n, m, x, y;
	while (t--)
	{
		cin >> n >> m >> x >> y;

		int cross_time = 0;

		for (auto index = 1; index <= n; index++)
		{
			int vertical_pos;
			cin >> vertical_pos;
			if (vertical_pos <= y)
				cross_time++;
		}

		for (auto index = 1; index <= m; index++)
		{
			int horizontal_pos;
			cin >> horizontal_pos;
			if (horizontal_pos <= x)
				cross_time++;
		}

		cout << cross_time << endl;
	}

	return 0;
}