#include <iostream>
using namespace std;

int main() {
	unsigned char test_cases;
	cin >> test_cases;
	while (test_cases--) {
	    unsigned short N, X;
	    cin >> N >> X;
	    unsigned short curr_people = X;
	    while (N--) {
	        int people_rn;
	        cin >> people_rn;
	        curr_people += people_rn;
	        if (curr_people > X) {
	            X = curr_people;
	        }
	    }
	    
	    cout << max << '\n';
	}
	return 0;
}
