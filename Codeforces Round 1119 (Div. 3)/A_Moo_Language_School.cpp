#include <iostream>
#include <string>
using namespace std;

int main()
{
	int t;
	cin >> t;
	while (t--)
	{
		int n, k;
		cin >> n >> k;
		string s;
		cin >> s;
		int count = 0;
		int result = 0;
		for (int i = 0; i < n; i++)
		{
			if (i % k == 0)
			{
				count = 0;
			}
			if (s[i] == '1')
			{
				count++;
			}
			if (count == k)
			{
				result++;
			}
		}
		cout << result << endl;
	}
	return 0;
}