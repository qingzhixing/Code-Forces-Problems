#include <iostream>
#include <set>
#include <string>
using namespace std;

const int MAX_N = 110;
const int MAX_M = 5e4;

void solution()
{
	int n, m;
	cin >> n >> m;

	set<char> exist;
	for (int i = 1; i <= n; i++)
	{
		string s;
		cin >> s;
		exist.insert(tolower(s[0]));
	}

	bool failed = false;
	for (int i = 1; i <= m; i++)
	{
		string q;
		cin >> q;

		if (failed)
		{
			continue;
		}

		for (auto ch : q)
		{
			ch = tolower(ch);
			if (!exist.count(ch))
			{
				failed = true;
				break;
			}
		}
	}

	if (failed)
	{
		cout << "NO" << endl;
	}
	else
	{
		cout << "YES" << endl;
	}
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