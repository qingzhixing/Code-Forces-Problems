#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
using namespace std;

const int MAX_N = 110;

void solution()
{
	int n;
	string s;
	cin >> n >> s;
	vector<int> S, K;
	for (int i = 1; i <= n; i++)
	{
		if (s[i - 1] == '1')
		{
			S.push_back(i);
		}
		else
		{
			K.push_back(i);
		}
	}
	// S 只有一个元素
	if (S.size() == 1)
	{
		cout << 0 << endl;
		return;
	}

	if (S.size() == 2)
	{
		cout << 1 << endl;
		return;
	}

	cout << S.back() - S.front() << endl;
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