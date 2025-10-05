#include <iostream>
using namespace std;

void Solution()
{
	int n, number;
	cin >> n;
	int operation_count = 0;
	bool odd_minus_one = false;
	while (n--)
	{
		cin >> number;
		if (number == 0)
		{
			operation_count++;
			continue;
		}
		if (number == -1)
		{
			odd_minus_one = !odd_minus_one;
		}
	}
	if (odd_minus_one)
		operation_count += 2;

	cout << operation_count << endl;
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