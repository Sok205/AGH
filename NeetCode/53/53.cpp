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
    vector<vector<int>> solution;

    void dfs(TreeNode* root, int level){
        if (!root){
            return;
        }

        if (solution.size() <= level) {
            solution.push_back({});
        }

        solution[level].push_back(root->val);

        dfs(root->left, level + 1);
        dfs(root->right, level + 1);
    }
public:
    vector<vector<int>> levelOrder(TreeNode* root) {

        dfs(root, 0);
        return solution;
    }
};
