#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

bool ArrayCompare(vector<int> array1, vector<int> array2)
{
	int len1 = array1.size();
	int len2 = array2.size();
	int len = min(len1, len2);
	for (auto index = 0; index < len; index++)
	{
		if (array1[index] != array2[index])
		{
			return array1[index] < array2[index];
		}
	}
	return len1 > len2;
}

void Solution()
{
	int n{};
	cin >> n;
	vector<vector<int>> arrays;

	int answer_length{};
	while (n--)
	{
		int k{};
		cin >> k;

		answer_length = max(answer_length, k);

		vector<int> array;
		while (k--)
		{
			int value;
			cin >> value;
			array.push_back(value);
		}
		arrays.push_back(array);
	}

	sort(arrays.begin(), arrays.end(), ArrayCompare);

	for (auto index = 0; index < answer_length; index++)
	{
		for (const auto array : arrays)
		{
			if (array.size() > index)
			{
				cout << array[index] << ' ';
				break;
			}
		}
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