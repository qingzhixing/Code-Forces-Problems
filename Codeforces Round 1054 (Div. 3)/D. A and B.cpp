#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
using namespace std;

void Solution()
{
	int n;
	cin >> n;
	string str;
	cin >> str;

	vector<int> a_dist, b_dist;
	for (int index = 0; index < n; index++)
	{
		char ch = str[index];
		if (ch == 'a')
		{
			a_dist.push_back(index - a_dist.size());
		}
		else
		{
			b_dist.push_back(index - b_dist.size());
		}
	}

	if (!a_dist.size() || !b_dist.size())
	{
		cout << 0 << endl;
		return;
	}

	sort(a_dist.begin(), a_dist.end());
	sort(b_dist.begin(), b_dist.end());

	long long a_L = a_dist[a_dist.size() / 2];
	long long b_L = b_dist[b_dist.size() / 2];

	long long a_cost = 0, b_cost = 0;

	for (int index = 0; index < a_dist.size(); index++)
	{
		a_cost += abs(a_dist[index] - a_L);
	}

	for (int index = 0; index < b_dist.size(); index++)
	{
		b_cost += abs(b_dist[index] - b_L);
	}

	cout << min(a_cost, b_cost) << endl;
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