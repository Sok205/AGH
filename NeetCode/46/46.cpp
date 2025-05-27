/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */

class Solution {
private:
    TreeNode* _invertTree(TreeNode* root){

        TreeNode* tmp = root->right;
        root->right = root->left;
        root->left = tmp;

        if(root->left){
            _invertTree(root->left);
        }

        if(root->right){
            _invertTree(root->right);
        }

        return root;
    }
public:
    TreeNode* invertTree(TreeNode* root) {
        if(root == nullptr){
            return root;
        }

        return _invertTree(root);
    }
};
