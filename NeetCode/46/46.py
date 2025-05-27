# Definition for a binary tree node.
from typing import Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def invertTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:

        if not root:
            return root

        def _invertTree(root: TreeNode) -> TreeNode:

            tmp = root.right
            root.right = root.left
            root.left = tmp

            if root.right:
                _invertTree(root.right)
            if root.left:
                _invertTree(root.left)

            return root

        return _invertTree(root)