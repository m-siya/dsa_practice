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
    string str = "";
public:
    string tree2str(TreeNode* root) {
        _tree2str(root);
        return str;
    }

    void _tree2str(TreeNode* root) {
        str += to_string(root->val);
        if (root->right) {
            str += "(";
            if (root->left)
                tree2str(root->left);
            str += ")";

            str += "(";
            tree2str(root->right);
            str += ")";
        } else if (root->left) {
            str += "(";
            tree2str(root->left);
            str += ")";
        }
    }
};
