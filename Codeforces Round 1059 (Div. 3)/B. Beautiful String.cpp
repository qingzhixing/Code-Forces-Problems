#include <iostream>
#include <string>
#include <vector>
using namespace std;

void Solution()
{
	int n;
	string str;
	cin >> n;
	cin >> str;

	vector<int> zero_idx;
	for (int i = 0; i < n; i++)
	{
		if (str[i] == '0')
			zero_idx.push_back(i);
	}

	cout << zero_idx.size() << "\n";
	for (auto idx : zero_idx)
	{
		cout << idx + 1 << " ";
	}
}

int main()
{
	int t;
	cin >> t;
	while (t--)
		Solution();
	return 0;
}