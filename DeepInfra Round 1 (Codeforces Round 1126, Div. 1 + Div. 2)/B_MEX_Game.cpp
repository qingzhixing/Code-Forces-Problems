#include <iostream>
#include <vector>
#include <map>
using namespace std;

const int MAX_N = 2e5 + 10;
const int MAX_M = 2e5;
const int INF = MAX_M + 10;

void solution()
{
	int n, k;
	cin >> n >> k;

	vector<int> a(n);
	map<int, int> counter;
	for (auto &item : a)
	{
		cin >> item;
		counter[item]++;
	}

	int number = 0;
	// 找到第一个 number < 2 * k
	while ((counter.find(number) != counter.end()) && counter[number] >= 2 * k)
	{
		number++;
	}

	// 这个数字限制了 B 但是无法限制 A
	if (counter[number] == 2 * k - 1)
	{
		cout << "YES" << endl;
	}
	else
	{
		// A B 一起被限制了
		cout << "NO" << endl;
	}
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