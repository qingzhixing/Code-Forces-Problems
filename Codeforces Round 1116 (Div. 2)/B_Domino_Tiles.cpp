#include <iostream>
#include <string>
using namespace std;

void solution()
{
	int n;
	string s;
	cin >> n >> s;

	int first_odd_pos = -1;
	int first_even_pos = -1;

	for (int i = 0; i < n; i++)
	{
		if (s[i] == '?')
		{
			continue;
		}

		if (i % 2 == 0)
		{
			// even
			if (first_even_pos == -1)
			{
				first_even_pos = i;
			}
			else
			{
				int step = (i - first_even_pos) / 2;
				// 无解
				if ((step % 2) && s[i] == s[first_even_pos] || !(step % 2) && s[i] != s[first_even_pos])
				{
					cout << 0 << endl;
					return;
				}
			}
		}
		else
		{
			// odd
			if (first_odd_pos == -1)
			{
				first_odd_pos = i;
			}
			else
			{
				int step = (i - first_odd_pos) / 2;
				// 无解
				if ((step % 2) && s[i] == s[first_odd_pos] || !(step % 2) && s[i] != s[first_odd_pos])
				{
					cout << 0 << endl;
					return;
				}
			}
		}
	}

	int odd = (first_odd_pos != -1) ? 1 : 2;
	int even = (first_even_pos != -1) ? 1 : 2;
	cout << odd * even << endl;
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