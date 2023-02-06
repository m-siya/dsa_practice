#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    vector<vector<string>> all_matrices;

    bool isValidMove(vector<string> &matrix, int x, int y) {
        int n = matrix.size();
        for (int i = 0; i < n; i++) {
            // no need to check for row caz whats done is done
            if (matrix[i][y] == 'Q') // check col for queen
                return false;
            if (x - i >= 0 && y - i >= 0 &&
                matrix[x - i][y - i] == 'Q') // check left diagonal
                return false;
            if (x - i >= 0 && y + i < n &&
                matrix[x - i][y + i] == 'Q') // check right diagonal
                return false;
        }
        return true;
    }

    void insertQueenAt(vector<string> &matrix, int x = 0) {
        if (matrix.size() == x) {
            all_matrices.push_back(matrix);
            return;
        }
        for (int y = 0; y < matrix.size(); y++) { // row = x and col to look for
            if (isValidMove(matrix, x, y)) {
                matrix[x][y] = 'Q';           // put queen
                insertQueenAt(matrix, x + 1); // check and return
                matrix[x][y] = '.';           // die out of regret
            } else {
                matrix[x][y] = '.'; // invalid move
            }
        }
    }

    auto solveNQueens(int n) {
        vector<string> matrix(n, string(n, '.'));
        insertQueenAt(matrix);
        return all_matrices;
    }
};