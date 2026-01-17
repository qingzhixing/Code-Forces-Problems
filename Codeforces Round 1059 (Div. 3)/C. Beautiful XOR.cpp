#include <iostream>
#include <vector>

using namespace std;

#define GET_BIT(value, idx) ((value >> idx) & 1)

int log2(int value)
{
	int result = 0;
	while (value)
	{
		value >>= 1;
		result++;
	}
	return result;
}

void Solution()
{
	int a, b;
	cin >> a >> b;

	int log2_b = log2(b);
	int log2_a = log2(a);

	if (log2_b > log2_a)
	{
		cout << "-1" << endl;
		return;
	}

	int bit_idx = 0;

	vector<int> result;

	while (bit_idx < log2_a)
	{
		if (GET_BIT(a, bit_idx) != GET_BIT(b, bit_idx))
		{
			result.emplace_back(1 << bit_idx);
		}
		bit_idx++;
	}
	cout << result.size() << endl;
	for (const auto &val : result)
	{
		cout << val << " ";
	}
	cout << endl;
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