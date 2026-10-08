#include <iostream>
using namespace std;

const int MAX_N = 0;
const int MAX_M = 0;

void solution()
{
	int x, y, r;
	cin >> x >> y >> r;
	printf("%d %d\n", x + r, y);
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