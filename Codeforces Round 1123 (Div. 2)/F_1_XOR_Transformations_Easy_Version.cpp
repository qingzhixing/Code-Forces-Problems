#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

const int MAX_N = 2e3 + 10;

int n, q;

vector<int> transform(const vector<int> &array)
{
	vector<int> temp;
	int len = array.size();
	for (int i = 0; i < len; i++)
	{
		for (int j = 0; j < i; j++)
		{
			temp.push_back(array[i] ^ array[j]);
		}
	}

	sort(temp.begin(), temp.end());

	return vector<int>(temp.begin(), temp.begin() + n);
}

void Solution()
{
	cin >> n >> q;
	vector<int> a;

	for (int i = 1; i <= n; i++)
	{
		int number;
		cin >> number;
		a.push_back(number);
	}

	vector<int> ans;

	// 计算第 0 次变换的答案
	sort(a.begin(), a.end());
	ans.push_back(a.back() - a.front());

	// 一直变换 a 直到 全为 0
	while (a.back() != 0)
	{
		a = transform(a);
		ans.push_back(a.back() - a.front());
	}

	// 变为全 0 的步骤数
	int zero_step = ans.size() - 1;

	while (q--)
	{
		int x;
		cin >> x;
		if (x >= zero_step)
		{
			cout << 0 << endl;
		}
		else
		{
			cout << ans[x] << endl;
		}
	}
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