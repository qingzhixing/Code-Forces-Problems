#include <iostream>
#include <unordered_map>
#include <vector>
using namespace std;

const int MAX_N = 3e5 + 10;
const int MAX_M = 3e5 + 10;

vector<int> get_factors(int number)
{
	if (number < 2)
	{
		return {};
	}

	vector<int> factors;
	for (int i = 2; i <= number / i; i++)
	{
		if (number % i == 0)
		{
			factors.push_back(i);
			while (number % i == 0)
			{
				number /= i;
			}
		}
	}

	if (number > 1)
	{
		factors.push_back(number);
	}

	return factors;
}

void Solution()
{
	int n, x;
	cin >> n >> x;

	unordered_map<int, long long> factor_sum;

	for (int i = 1; i <= n; i++)
	{
		int num;
		cin >> num;
		auto factors = get_factors(num);
		for (auto factor : factors)
		{
			factor_sum[factor] += num;
		}
	}

	long long result = 0;
	auto factors = get_factors(x);
	for (auto factor : factors)
	{
		result = max(result, factor_sum[factor]);
	}

	cout << result << endl;
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