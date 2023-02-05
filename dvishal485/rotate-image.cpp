#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    int n;
    // void swap_int(int& a, int& b){ a^=b; b^=a; a^=b; }
    void move(vector<vector<int>>& matrix, int diag){
        for(int i = diag; i < n-diag-1; i++){
            int tmp = matrix[diag][i];
            matrix[diag][i] = matrix[n-i-1][diag];
            matrix[n-i-1][diag] = matrix[n-diag-1][n-i-1];
            matrix[n-diag-1][n-i-1] = matrix[i][n-diag-1];
            matrix[i][n-diag-1] = tmp;
        }
    }

   void rotate(vector<vector<int>>& matrix) {
    n = matrix.size();
        for (int diag = 0; diag < n/2; diag++) {
            move(matrix, diag);
        }
    }

};