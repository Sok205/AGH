class Solution:
    def letterCombinations(self, digits: str) -> List[str]:
        res = []
        hashMap = {
            '2': ['a', 'b', 'c'],
            '3': ['d', 'e', 'f'],
            '4': ['g', 'h', 'i'],
            '5': ['j', 'k', 'l'],
            '6': ['m', 'n', 'o'],
            '7': ['p', 'q', 'r', 's'],
            '8': ['t', 'u', 'v'],
            '9': ['w', 'x', 'y', 'z']
        }

        def addDigits(i: int, cur: str) -> None:
            if len(cur) == len(digits):
                res.append(cur)
                return
            for c in hashMap[digits[i]]:
                addDigits(i + 1, cur + c)

        if digits:
            addDigits(0, "")

        return res