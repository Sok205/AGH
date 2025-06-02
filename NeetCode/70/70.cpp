class Solution {

private:
    vector<vector<int>> res;
    void dfs(int i, vector<int>& candidates, int target, vector<int>& cur, int total){
        if(total == target){
            res.push_back(cur);
            return;
        }

        if(total > target || i == candidates.size()){
            return;
        }
        cur.push_back(candidates[i]);
        dfs(i + 1, candidates, target, cur, total + candidates[i]);
        cur.pop_back();

        while(i + 1 < candidates.size() && candidates[i] == candidates[i + 1]){
            i++;
        }
        dfs(i + 1,candidates, target, cur, total);
    }
public:
    vector<vector<int>> combinationSum2(vector<int>& candidates, int target) {
        res.clear();
        sort(candidates.begin(), candidates.end());
        vector<int> cur;
        dfs(0,candidates,target, cur, 0);
        return res;
    }
};
