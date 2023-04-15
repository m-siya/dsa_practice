struct TreeNode {
     int val;
     TreeNode *left;
     TreeNode *right;
     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

class Solution {
public:
    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
        int smaller = p->val < q->val ? p->val : q->val;
        int bigger = p->val == smaller ? q->val : p->val;
        while (root != nullptr) {
            if (smaller <= root->val && root->val <= bigger) {
                return root;
            }
            if (smaller <= root->val && bigger <= root->val) {
                root = root->left;
            } else {
                root = root->right;
            }
        }

        return nullptr;
    }
};

int main() {

}