// SOLVED

#include <iostream>

using namespace std;

void display_table(unsigned int n, unsigned int k) {
    for (unsigned i = 1; i <= n; i++) {
        for (unsigned j = 0; j < k-1; j++) {
            cout << i + j*n << ' ';
        }
        cout << i + (k-1)*n << '\n'; 
    }
}

void solution(unsigned int n, unsigned int k) {
    if (n % 2 == 0 || k == 1) {
        cout << "YES\n";
        display_table(n, k);
    } else {
        cout << "NO\n";
    }
}

int main() {
    int test_cases;
    cin >> test_cases;

    for (int i = 0; i < test_cases; i++) {
        unsigned int n,k;
        cin >> n >> k;
        solution(n,k);
    }
}