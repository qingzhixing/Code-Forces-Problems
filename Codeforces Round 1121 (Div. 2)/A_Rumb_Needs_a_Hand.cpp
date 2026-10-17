#include <iostream>
#include <vector>
using namespace std;

const int MAX_N = 110;

void solution()
{
	int n;
	cin >> n;
	vector<int> uncorrect;
	for (int i = 1; i <= n; i++)
	{
		int a;
		cin >> a;
		if (a != i)
		{
			uncorrect.push_back(a);
		}
	}
	for (int i = uncorrect.size() - 2; i >= 0; i--)
	{
		if (uncorrect[i] <= uncorrect[i + 1])
		{
			cout << "NO" << endl;
			return;
		}
	}
	cout << "YES" << endl;
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