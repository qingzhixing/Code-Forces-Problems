#include <iostream>
#include <string>
using namespace std;

const int MAX_N = 0;
const int MAX_M = 0;

void Solution()
{
	int n;
	string s;
	char c;
	cin >> n >> c;
	cin >> s;
	int result = 0;
	for (int i = 0; i < n / 2; i++)
	{
		if (s[i] != s[n - i - 1])
		{
			if (s[i] != c)
			{
				result++;
			}
			if (s[n - i - 1] != c)
			{
				result++;
			}
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