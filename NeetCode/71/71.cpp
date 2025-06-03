class Solution {

private:
    vector<vector<int>> res;

    void backtrack(vector<int>& perm, vector<int>& nums, vector<bool>& check){
        if(perm.size() == nums.size()){
            res.push_back(perm);
            return;
        }

        for(int i = 0; i < nums.size(); ++i){
            if (!check.at(i))
            {
                perm.push_back(nums.at(i));
                check.at(i) = true;
                backtrack(perm,nums,check);
                perm.pop_back();
                check.at(i) = false;
            }

        }
    }
public:
    vector<vector<int>> permute(vector<int>& nums) {
        vector<bool> check(nums.size(), false);
        vector<int> perm;
        backtrack(perm, nums, check);
        return res;
    }

};
