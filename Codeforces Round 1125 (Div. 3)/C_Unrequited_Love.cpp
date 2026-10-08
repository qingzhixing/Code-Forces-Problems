#include <iostream>
#include <vector>
#include <map>
#include <set>
using namespace std;

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

	map<long long, set<int>> love_notes;

	for (int i = 0; i < n - 4; i++)
	{
		const auto love = a[i] + a[i + 2] - a[i + 4];
		love_notes[love].insert(i);
	}

	long long result = 0LL;

	for (const auto &[love_value, notes] : love_notes)
	{
		const long long cnt = notes.size();

		result += (cnt * (cnt - 1)) / 2;

		for (const auto &note : notes)
		{
			if (notes.find(note + 2) != notes.end())
			{
				result--;
			}
			if (notes.find(note + 4) != notes.end())
			{
				result--;
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
		solution();
	}
	return 0;
}