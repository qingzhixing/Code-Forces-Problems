#include <iostream>
#include <vector>
#include <map>
using namespace std;

#error TODO: Wrong Answer.

const int MAX_N = 2e5 + 10;

void solution()
{
	int n;

	cin >> n;

	vector<int> a(n);

	for (int i = 0; i < n; i++)
	{
		cin >> a[i];
	}

	map<long long, vector<int>> love_notes;

	for (int i = 0; i < n - 4; i++)
	{
		const auto love = a[i] + a[i + 2] - a[i + 4];
		love_notes[love].push_back(i);
	}

	long long result = 0LL;

	for (const auto &[love_value, notes] : love_notes)
	{
		}

	cout << result << endl;
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