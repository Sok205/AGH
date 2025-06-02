class Solution:
    def combinationSum(self, nums: List[int], target: int) -> List[List[int]]:
        res = []
        curNum = []

        def dfs(i: int) -> None:
            suma = sum(curNum)
            if suma > target or i >= len(nums):
                return
            elif suma == target:
                res.append(curNum.copy())
                return
            curNum.append(nums[i])
            dfs(i)
            curNum.pop()
            dfs(i + 1)

        dfs(0)
        return res