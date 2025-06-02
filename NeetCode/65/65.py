class Solution:
    def leastInterval(self, tasks: List[str], n: int) -> int:

        hashAlph = [0] * 26

        for char in tasks:
            hashAlph[ord(char) - 64] += 1

        hashAlph.sort()
        maxf = hashAlph[25]
        idle = (maxf - 1) * n

        for i in range(24, -1, -1):
            idle -= min(maxf - 1, hashAlph[i])
        return max(0, idle) + len(tasks)
