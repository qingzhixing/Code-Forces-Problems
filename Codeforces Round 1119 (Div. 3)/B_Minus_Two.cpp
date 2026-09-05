#include <iostream>
using namespace std;

int main()
{
	int t;
	cin >> t;
	while (t--)
	{
		int n;
		cin >> n;
		int mod_4[5] = {};
		while (n--)
		{
			int num;
			cin >> num;
			mod_4[num % 4]++;
		}
		cout << max(mod_4[0], max(mod_4[2], mod_4[1] + mod_4[3])) << endl;
	}
	return 0;
}