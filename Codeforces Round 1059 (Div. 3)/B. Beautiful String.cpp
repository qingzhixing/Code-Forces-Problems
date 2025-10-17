#include <iostream>
#include <string>
#include <vector>
using namespace std;

string int_to_binary_str(int number, int digit)
{
	string result;
	for (int bit = digit - 1; bit >= 0; bit--)
	{
		result += ((number >> bit) & 1) + '0';
	}
	return result;
}

bool is_number_increase(int number)
{
	while (number)
	{
		if (!number & 1)
			return false;
		number >>= 1;
	}
	return true;
}

pair<bool, pair<string, vector<int>>> remove_str(string origin, string sub)
{
	string result{};
	vector<int> idx;
	if (origin.size() > sub.size())
	{
		return {false, {"", {}}};
	}

	int sub_ptr = 0;
	for (int origin_ptr = 0; origin_ptr < origin.size(); origin_ptr++)
	{
		if (sub_ptr < sub.size() && sub[sub_ptr] == origin[origin_ptr])
		{
			idx.push_back(origin_ptr + 1);
			sub_ptr++;
		}
		else
		{
			result += origin_ptr;
		}
	}

	if (sub_ptr < sub.size())
	{
		return {false, {"", {}}};
	}

	return {true, {result, idx}};
}

bool is_str_palindrome(string str)
{
	int i = 0, j = str.size() - 1;
	while (i <= j)
	{
		if (str[i] == str[j])
		{
			i++;
			j--;
		}
		else
		{
			return false;
		}
	}
	return true;
}

void Solution()
{
	int n;
	string str;
	cin >> n;
	cin >> str;

	if (is_str_palindrome(str))
	{
		cout << 0 << endl;
		return;
	}

	// 字串长度
	for (int len = 1; len <= n; len++)
	{
		int one_cnt = 0;
		for (int sub = 0; sub < (1 << len); sub = (1 << one_cnt++) - 1)
		{
			// if (!is_number_increase(sub))
			// {

			// 	printf("Unincrease\n");
			// 	continue;
			// }

			// printf("Increase\n");

			string sub_str = int_to_binary_str(sub, len);
			printf("sub_str: %s\n", sub_str.c_str());
			auto [ok, p_out] = remove_str(str, sub_str);
			auto [last_str, idx] = p_out;
			if (!ok)
			{
				continue;
			}
			printf("Last str: %s\n", last_str.c_str());
			if (!is_str_palindrome(last_str))
			{
				printf("not palindrome\n");
				continue;
			}
			cout << idx.size() << endl;
			for (auto single_idx : idx)
			{
				cout << single_idx << ' ';
			}
			cout << endl;
			return;
		}
	}
	cout << -1 << endl;
}

int main()
{
	int t;
	cin >> t;
	while (t--)
		Solution();
	return 0;
}