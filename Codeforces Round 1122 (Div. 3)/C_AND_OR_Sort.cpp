#include <iostream>
#include <string>
using namespace std;

const int MAX_N = 2e5 + 10;

int forward_1[MAX_N];
int backward_0[MAX_N];

void Solution()
{
	int n;
	string s;
	cin >> n >> s;

	int cnt_0 = 0, cnt_1 = 0;

	// 初始化每个节点前面有多少个 1
	for (int i = 0; i < n; i++)
	{
		forward_1[i] = cnt_1;
		if (s[i] == '1')
		{
			cnt_1++;
		}
	}

	// 初始化后面有多少个 0
	for (int i = n - 1; i >= 0; i--)
	{
		backward_0[i] = cnt_0;
		if (s[i] == '0')
		{
			cnt_0++;
		}
	}

	// 如果第一位是 1, 则只能把后面全部变成 0
	if (s[0] == '1')
	{
		cout << cnt_0 << endl;
		return;
	}

	int result = 1e9;

	// 如果末尾为 0 我们可以尝试将数列变为全 0
	if (s[n - 1] == '0')
	{
		result = min(result, cnt_1);
	}

	// 枚举每一个 1, 尝试把它前面的 1 变成 0, 后面的 0 变成 1
	// 只有 前面有 0 的 1 才能做到这点，所以只用枚举 1
	for (int i = 0; i < n; i++)
	{
		if (s[i] == '0')
		{
			continue;
		}
		result = min(result, forward_1[i] + backward_0[i]);
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