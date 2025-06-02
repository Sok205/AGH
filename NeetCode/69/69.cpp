class Solution {

private:
    vector<vector<int>> res;
    vector<int> curNum;

    void dfs(int i, vector<int>& nums, int target, int suma){
        if(suma > target || i >= nums.size()){
            return;
        }
        else if(suma == target){
            res.push_back(curNum);
            return;
        }
        curNum.push_back(nums.at(i));
        dfs(i, nums, target, suma + nums.at(i));
        curNum.pop_back();
        dfs(i + 1, nums, target, suma);
    }

public:
    vector<vector<int>> combinationSum(vector<int>& nums, int target) {
        dfs(0, nums, target, 0);
        return res;
    }
};
