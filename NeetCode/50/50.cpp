class Solution {
public:
    bool isSameTree(TreeNode* p, TreeNode* q) {
        if(!p && !q){
            return true;
        }
        if(p && q && p->val == q->val){
            return isSameTree(p->right, q->right) && isSameTree(p->left, q->left);
        }
        else{
            return false;
        }
    }
};