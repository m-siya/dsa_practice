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
public:
    vector<int> rightSideView(TreeNode* root) {
        vector<int> result;
        result.reserve(100);
        vector<TreeNode*> levelTraversal;
        levelTraversal.reserve(100);
        
        if (root != nullptr)
            levelTraversal.push_back(root);
        int i = 0;
        while (i < levelTraversal.size()) {
            int currSize = levelTraversal.size();
            for (; i < currSize; i++) {
                auto left = levelTraversal[i]->left;
                auto right = levelTraversal[i]->right;
                if (left)
                    levelTraversal.push_back(left);
                if (right)
                    levelTraversal.push_back(right);
            }
            result.push_back(levelTraversal[currSize-1]->val);
        }

        return result;
    }
};