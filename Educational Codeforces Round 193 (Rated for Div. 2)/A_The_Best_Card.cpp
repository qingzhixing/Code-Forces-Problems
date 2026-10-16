#include <iostream>
using namespace std;

const int MAX_N = 3e6 + 10;

bool is_prime(int n)
{
	if (n < 2)
	{
		return false;
	}
	for (int i = 2; i <= n / i; i++)
	{
		if (n % i == 0)
		{
			return false;
		}
	}
	return true;
}

void solution()
{
	int n;
	cin >> n;
	cout << (is_prime(n + 1) ? "YES" : "NO") << endl;
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