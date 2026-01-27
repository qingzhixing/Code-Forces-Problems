#include <iostream>
#include <vector>
using namespace std;

int number_limit;

int gcd(int a, int b)
{
	if (b == 0)
		return a;
	return gcd(b, a % b);
}

int main()
{
	int w, h, d, n;
	cin >> w >> h >> d >> n;

	int w_accumulator = 1, h_accumulator = 1, d_accumulator = 1;

	while (n > 1)
	{
		int gcd_wn = gcd(w, n);
		w /= gcd_wn;
		n /= gcd_wn;
		w_accumulator *= gcd_wn;

		int gcd_hn = gcd(h, n);
		h /= gcd_hn;
		n /= gcd_hn;
		h_accumulator *= gcd_hn;

		int gcd_dn = gcd(d, n);
		d /= gcd_dn;
		n /= gcd_dn;
		d_accumulator *= gcd_dn;

		if (gcd_wn == 1 && gcd_hn == 1 && gcd_dn == 1)
		{
			cout << -1 << endl;
			return 0;
		}
	}

	cout << w_accumulator - 1 << " " << h_accumulator - 1 << " " << d_accumulator - 1 << endl;
	return 0;
}