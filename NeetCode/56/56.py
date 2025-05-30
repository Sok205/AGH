class Solution:
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        def dfs(node: Optional[TreeNode], minVal: float, maxVal: float) -> bool:
            if not node:
                return True

            if not (minVal < node.val < maxVal):
                return False

            return (dfs(node.left, minVal, node.val) and
                    dfs(node.right, node.val, maxVal))

        return dfs(root, float('-inf'), float('inf'))