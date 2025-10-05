#include <iostream>
#include <algorithm>
#include <cstring>
using namespace std;

static const int MAX_N = (2e5 + 10);

int n;
int nums[MAX_N];

void Solution()
{
	memset(nums, 0, sizeof(nums));
	cin >> n;
	for (int i = 1; i <= n; i++)
	{
		cin >> nums[i];
	}
	sort(nums + 1, nums + 1 + n);

	int max_diff = -2147483648;

	for (int i = 2; i <= n; i += 2)
	{
		max_diff = max(max_diff, nums[i] - nums[i - 1]);
	}

	cout << max_diff << endl;
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