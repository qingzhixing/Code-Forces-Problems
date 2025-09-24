#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

static const int MAX_N = (2e5 + 10);

int main()
{

	int t, n;

	cin >> t;
	while (t--)
	{
		cin >> n;

		vector<int> odd_array{};

		long long even_sum = 0;

		for (auto i = 1; i <= n; i++)
		{
			int number;
			cin >> number;
			if (number & 1)
			{
				odd_array.push_back(number);
			}
			else
			{
				even_sum += number;
			}
		}

		long long answer = 0;
		if (odd_array.size())
		{
			answer += even_sum;
		}

		int odd_len = odd_array.size();

		sort(odd_array.begin(), odd_array.end());

		for (int index = odd_len - 1; index > (odd_len / 2 - 1); index--)
		{
			answer += odd_array[index];
		}

		cout << answer << endl;
	}
	return 0;
}