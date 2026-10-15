#include <iostream>
#include <string>
using namespace std;

void solution()
{
	int n, k;
	string s;
	cin >> n >> k >> s;

	int red = 0, blue = 0;
	for (int i = 0; i < 2 * n; i++)
	{
		if (s[i] != '1')
		{
			continue;
		}

		int nxt = s[(i + 1) % (2 * n)];
		if (nxt == '1')
		{
			if (i % 2 == 0)
			{
				blue++;
			}
			else
			{
				red++;
			}
		}
		else
		{
			if (i % 2 == 0)
			{
				red++;
			}
			else
			{
				blue++;
			}
		}
	}

	cout << red << ' ' << blue << endl;
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