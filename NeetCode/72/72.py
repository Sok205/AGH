class Solution:
    def subsetsWithDup(self, nums: List[int]) -> List[List[int]]:
        res = []
        curr = []

        def dfs(i: int) -> None:
            if i >= len(nums):
                x = sorted(curr)
                if x not in res:
                    res.append(x)
                return

            curr.append(nums[i])
            dfs(i + 1)
            curr.pop()
            dfs(i + 1)

        dfs(0)

        return res

