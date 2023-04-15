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
    int sumNumbers(TreeNode* root) {
        return _sumNumbers(root, 0);
    }

    int _sumNumbers(TreeNode* root, int curr_sum) {
        if (root->left == nullptr && root->right == nullptr)
            return curr_sum*10 + root->val;
        
        int leftSum = 0;
        if (root->left)
            leftSum = _sumNumbers(root->left, curr_sum * 10 + root->val);
        int rightSum = 0;
        if (root->right)
            rightSum = _sumNumbers(root->right, curr_sum * 10 + root->val);
        
        return leftSum + rightSum;
    }
};