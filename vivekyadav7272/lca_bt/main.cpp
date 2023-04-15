#include <bits/stdc++.h>

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

class Solution {
public:
    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
        if (!root || root->val == p->val || root->val == q->val)
            return root;
        
        TreeNode* smaller = lowestCommonAncestor(root->left, p, q);
        TreeNode* bigger = lowestCommonAncestor(root->right, p, q);

        if (smaller && bigger) {
            return root;
        } else {
            return smaller || bigger;
        }
    }

    bool find(TreeNode* root, TreeNode* p, TreeNode* q) {

    }
};