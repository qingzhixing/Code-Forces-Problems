#include <iostream>
#include <bit>
using namespace std;

const int MAX_N = 2e5 + 10;

int n, q;
int a[MAX_N];

bool even_popcount(int x)
{
	return popcount((unsigned int)(x)) % 2 == 0;
}

void Solution()
{
	cin >> n >> q;

	int result = 0;
	for (int i = 1; i <= n; i++)
	{
		cin >> a[i];
		result += even_popcount(a[i]);
	}

	cout << result << ' ';

	while (q--)
	{
		int p, x;
		cin >> p >> x;
		result -= even_popcount(a[p]);
		result += even_popcount(x);
		a[p] = x;
		cout << result << ' ';
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