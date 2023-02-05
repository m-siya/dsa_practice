#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    // void show(vector<vector<int>>& matrix){
    //     for (int i=0; i<matrix.size(); i++){
    //         for (int j=0; j<matrix[0].size(); j++)
    //             cout << matrix[i][j] << ' ';
    //         cout << '\n';
    //     }
    //     cout << '\n';
    // }
    void setZeros(vector<vector<int>>& matrix, int row, int col){
        for(int i=0; i<row; i++)
            matrix[i][col] = 0;
        for(int i=0; i<col; i++)
            matrix[row][i] = 0;
    }
    void setZeroes(vector<vector<int>>& matrix) {
        int rows = matrix.size(), cols = matrix[0].size();
        bool first_row_null = false;
        for(auto i : matrix[0]) if(i==0) {first_row_null = true; break;}
        for(int i=1; i<rows; i++){
            if(!matrix[i][0]) matrix[0][0] = 0;
            for(int j=1; j<cols; j++){
                // show(matrix);
                if(matrix[i][j]==0)
                    setZeros(matrix, i, j);
                else if(!matrix[i][0] || !matrix[0][j]) matrix[i][j] = 0;
            }
        }
        if(!matrix[0][0]) for(int i=1; i<rows; i++) matrix[i][0] = 0;
        if(first_row_null) for(int& i : matrix[0]) i = 0;
    }
};