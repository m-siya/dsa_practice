#include <iostream>

using namespace std;


void solution(int n) {

    for (int offset = 0; offset < n; offset++) {

        int i = 0;
        for (; i < n/3; i++) 
        {
            cout << n - 2*i << ' ' << n - 2*i - 1 << ' ' << (i + offset) % (n/3) + 1;
        }

        int leftover = n % 3;

        if (1 <= leftover) {
            cout << ' ' << n - 2*i;
        }

        if (2 <= leftover) {
            cout << ' ' << n - 2*i - 1;
        }

        cout << '\n';
    }
}

int main()
{
    int test_cases;
    cin >> test_cases;
    
    while (test_cases--) {
        int n;
        cin >> n;
        solution(n);
    }
}