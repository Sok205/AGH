"""
AVL Tree Implementation in Python
"""
from DSA import Node, print_tree


class AVLTree:

    def __init__(self):
        self.root = None

    def get_height(self, node):
        if not node:
            return 0
        return node.height

    def get_balance(self, node):
        if not node:
            return 0
        return self.get_height(node.left) - self.get_height(node.right)

    def right_rotate(self, z):
        y = z.left
        t3 = y.right

        y.right = z
        z.left = t3

        z.height = 1 + max(self.get_height(z.left), self.get_height(z.right))
        y.height = 1 + max(self.get_height(y.left), self.get_height(y.right))

        return y

    def left_rotate(self, z):
        y = z.right
        t3 = y.left

        y.left = z
        z.right = t3

        z.height = 1 + max(self.get_height(z.left), self.get_height(z.right))
        y.height = 1 + max(self.get_height(y.left), self.get_height(y.right))

        return y

    def insert(self, val):
        def _insert(node, val):
            if not node:
                return Node(val)
            elif val < node.val:
                node.left = _insert(node.left, val)
            else:
                node.right = _insert(node.right, val)

            node.height = 1 + max(self.get_height(node.left), self.get_height(node.right))
            balance = self.get_balance(node)

            # Left-Left Case
            if balance > 1 and val < node.left.val:
                return self.right_rotate(node)

            # Right-Right Case
            if balance < -1 and val > node.right.val:
                return self.left_rotate(node)

            # Left-Right Case
            if balance > 1 and val > node.left.val:
                node.left = self.left_rotate(node.left)
                return self.right_rotate(node)

            # Right-Left Case
            if balance < -1 and val < node.right.val:
                node.right = self.right_rotate(node.right)
                return self.left_rotate(node)

            return node

        self.root = _insert(self.root, val)

    def inorder_traverse(self):
        def _inorder_traverse(node):
            if node:
                _inorder_traverse(node.left)
                print(node.val, end=" ")
                _inorder_traverse(node.right)

        _inorder_traverse(self.root)
        print("")


if __name__ == "__main__":
    tree = AVLTree()
    tree.insert(10)
    tree.insert(20)
    tree.insert(30)
    tree.insert(40)
    tree.insert(50)

    print_tree(tree)


