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
    int rootVal;
    int res = 0;

    void dfs(TreeNode* root, int cMax)
    {
        if(!root){
            return;
        }

        if(root->val >= cMax){
            res ++;
        }

        int maxVal = max(cMax, root->val);

        if(root->right){dfs(root->right, maxVal);}
        if(root->left){dfs(root->left, maxVal);}
    }
public:
    int goodNodes(TreeNode* root) {
        rootVal = root->val;
        dfs(root, rootVal);
        return res;
    }
};
