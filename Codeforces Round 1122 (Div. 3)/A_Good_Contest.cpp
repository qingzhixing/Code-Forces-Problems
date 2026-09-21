#include <iostream>
using namespace std;

const int MAX_N = 0;
const int MAX_M = 0;

void Solution()
{
	int n;
	int a, b, c;
	cin >> n;
	cin >> a >> b >> c;
	auto m = min(a, min(b, c));
	cout << n - m << endl;
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