#include <iostream>
using namespace std;

void solution()
{
	int n;
	cin >> n;
	int avalable_easy = 0;
	int needed_easy = 0;
	for (int i = 1; i <= n; i++)
	{
		int d;
		cin >> d;
		if (i == 1 || i == n)
		{
			if (d == 1)
			{
				needed_easy++;
			}
		}
		else
		{
			if (d == 0)
			{
				avalable_easy++;
			}
		}
	}

	cout << ((!needed_easy || avalable_easy >= needed_easy) ? needed_easy : -1) << endl;
}

int main()
{
	int t;
	cin >> t;
	while (t--)
	{
		solution();
	}
	return 0;
}