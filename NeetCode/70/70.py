class Solution:
    def combinationSum2(self, nums: List[int], target: int) -> List[List[int]]:
        res = []
        subset = []

        def dfs(i):
            if i >= len(nums):
                if sum(subset) == target and sorted(subset) not in res:
                    res.append(sorted(subset.copy()))
                return
            subset.append(nums[i])
            dfs(i + 1)
            subset.pop()
            dfs(i + 1)

        dfs(0)

        return res
