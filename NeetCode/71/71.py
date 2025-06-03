class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        self.res = []
        self.backtrack([], nums, [False] * len(nums))
        return self.res

    def backtrack(self, perm: List[int], nums: List[int], check: List[int]) -> None:
        if len(perm) == len(nums):
            print(perm)
            self.res.append(perm[:])
            return

        for i in range(len(nums)):
            if not check[i]:
                perm.append(nums[i])
                check[i] = True
                self.backtrack(perm, nums, check)
                perm.pop()
                check[i] = False