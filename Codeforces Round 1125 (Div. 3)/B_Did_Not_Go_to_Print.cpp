#include <iostream>
#include <stack>
#include <bitset>
#include <string>
using namespace std;

const int MAX_N = 2e5 + 10;

bitset<MAX_N + 1> printed;

void solution()
{
	printed.reset();

	int n;
	int printed_cnt = 0;
	// id
	stack<int> memory;
	string command;

	cin >> n;
	cin >> command;
	for (int op_idx = 0; op_idx < n; op_idx++)
	{
		const auto id = op_idx + 1;
		const auto op = command[op_idx] - '0';

		if (op == 1)
		{
			memory.push(id);
			continue;
		}

		if (op == 2)
		{
			if (memory.empty())
			{
				printed[id] = true;
			}
			else
			{
				const auto print_id = memory.top();
				memory.pop();
				printed[print_id] = true;
			}
			printed_cnt++;
			continue;
		}

		if (op == 3)
		{
			printed[id] = true;
			printed_cnt++;
		}
	}

	cout << n - printed_cnt << endl;
	for (int i = 1; i <= n; i++)
	{
		if (!printed[i])
		{
			cout << i << ' ';
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
		solution();
	}
	return 0;
}