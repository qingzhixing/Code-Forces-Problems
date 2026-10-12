#include <iostream>
#include <vector>
#include <map>
using namespace std;

const int MAX_N = 1e3 + 10;
const int MAX_M = 1e9;

int digit_square_sum(int number)
{
	int result = 0;
	while (number)
	{
		const auto digit = (number % 10);
		result += digit * digit;
		number /= 10;
	}
	return result;
}

void solution()
{
	int n;
	cin >> n;
	vector<int> numbers(n);
	for (int i = 0; i < n; i++)
	{
		cin >> numbers[i];
	}

	for (int i = 1; i <= 50000; i++)
	{
		for (auto &number : numbers)
		{
			number = digit_square_sum(number);
		}
	}

	map<int, int> counters;

	for (auto &number : numbers)
	{
		counters[number]++;
	}

	int result = 0;
	for (const auto &[number, cnt] : counters)
	{
		result += cnt * (cnt - 1) / 2;
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