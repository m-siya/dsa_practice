#include <bits/stdc++.h>
using namespace std;

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class Solution {
    int maxTimeToInfect = 0;
public:
    int amountOfTime(TreeNode* root, int start) {
        _amountOfTime(root, start);
        return maxTimeToInfect;
    }
    int _amountOfTime(TreeNode* root, int patient0) {
        if (!root)
            return -1;

        if (root->val == patient0) {
            int leftInfectTime = infect(root->left, 0);
            int rightInfectTime = infect(root->right, 0);
            maxTimeToInfect = max(leftInfectTime, rightInfectTime);
            return 0;
        }

        int leftInfectTime = amountOfTime(root->left, patient0);
        if (leftInfectTime >= 0) {
            maxTimeToInfect += 1;
            maxTimeToInfect = infect(root->right, leftInfectTime + 1);
            return leftInfectTime + 1;
        }

        int rightInfectTime = amountOfTime(root->right, patient0);
        maxTimeToInfect = infect(root->left, rightInfectTime + 1);

        return rightInfectTime + 1;
    }

    int infect(TreeNode* root, int startTime) {
        if (!root)
            return startTime;
        return max(infect(root->left, startTime+1), infect(root->right, startTime+1));
    }
};