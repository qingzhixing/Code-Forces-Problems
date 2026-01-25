#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

void Solution()
{
	int n;
	cin >> n;
	vector<int> a(n + 10);
	for (int i = 1; i <= n; i++)
	{
		cin >> a[i];
	}
	sort(a.begin() + 1, a.begin() + 1 + n);
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