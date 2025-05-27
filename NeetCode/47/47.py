from typing import Optional

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        if not root:
            return 0
        maxK = 1

        def traverse(r: Optional[TreeNode], k):
            nonlocal maxK

            maxK = max(maxK, k)
            if r.left:
                traverse(r.left, k + 1)
            if r.right:
                traverse(r.right, k + 1)

            return

        traverse(root, 1)
        return maxK

