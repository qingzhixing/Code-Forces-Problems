#include <iostream>
#include <string>
#include <vector>
#include <array>
using namespace std;

const int MAX_N = 3e5 + 10;

void solution()
{
	string s;
	cin >> s;

	const int len = s.length();
	vector<int> s_to_origin_idx(len);

	// 不进行任何更改直接产生的子串
	string origin;

	char last = '\0';
	for (int i = 0; i < len; i++)
	{
		const auto &ch = s[i];
		if (last != ch)
		{
			origin.push_back(ch);
		}
		last = ch;
		s_to_origin_idx[i] = origin.size() - 1;
	}

	// exist[i][j] 表示开头为 i 长度为 j 的序列是否出现过
	vector<bool> exist(len + 10);
	int cnt = 0;

	const int origin_len = origin.length();
	exist[origin_len] = true;
	cnt++;

	// 枚举每一个点，看是否能改变
	for (int i = 1; i < len - 1; i++)
	{
		// 不可改变
		if (s[i - 1] != s[i + 1])
		{
			continue;
		}

		// 判断当前对 origin_len 的影响
		int changed_len = origin_len;

		if (s[i] == s[i - 1])
		{
			// 产生一个新的 0-1 对
			changed_len += 2;
		}
		else
		{
			// 删去一个 0-1 对
			changed_len -= 2;
		}
		if (!exist[changed_len])
		{
			cnt++;
			exist[changed_len] = true;
		}
	}

	cout << cnt << endl;
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