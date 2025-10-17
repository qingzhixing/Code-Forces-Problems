#include <iostream>
using namespace std;

void Solution()
{
	int n;
	int a[20]{};
	int pre_sum[20]{};
	cin >> n;
	for (int i = 1; i <= n; i++)
	{
		cin >> a[i];
		pre_sum[i] = pre_sum[i - 1] + a[i];
	}

	double max_average = -1;
	for (int start = 1; start <= n; start++)
	{
		for (int end = start; end <= n; end++)
		{
			max_average = max(max_average, (pre_sum[end] - pre_sum[start - 1]) * 1.0 / (end - start + 1) * 1.0);
		}
	}
	printf("%.0f\n", max_average);
}

int main()
{
	int t;
	cin >> t;
	while (t--)
		Solution();
	return 0;
}