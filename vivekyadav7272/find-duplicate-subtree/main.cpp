#include <bits/stdc++.h>
using namespace std;

struct TreeNode
{
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class Solution
{
    unordered_set<vector<int>> visited;

public:
    vector<TreeNode *> findDuplicateSubtrees(TreeNode *root)
    {
        vector<TreeNode *> tree_inorder;
        vector<TreeNode *> subtrees;
        _findDuplicateSubtrees(root, subtrees, tree_inorder);
        return subtrees;
    }

    void _findDuplicateSubtrees(TreeNode *root, vector<TreeNode *> &subtrees, vector<TreeNode *> &tree_inorder)
    {
        if (root == nullptr)
            return;

        int offset = tree_inorder.size();

        _findDuplicateSubtrees(root->left, subtrees, tree_inorder);
        tree_inorder.push_back(root);
        _findDuplicateSubtrees(root->right, subtrees, tree_inorder);
    }
};