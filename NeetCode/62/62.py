import heapq
from typing import List

class Solution:
    def lastStoneWeight(self, stones: List[int]) -> int:
        stoneHeap = [-s for s in stones]
        heapq.heapify(stoneHeap)

        while len(stoneHeap) > 1:

            f = heapq.heappop(stoneHeap)
            s = heapq.heappop(stoneHeap)

            if s > f:
                heapq.heappush(stoneHeap, f - s)

        stoneHeap.append(0)
        return abs(stoneHeap[0])
