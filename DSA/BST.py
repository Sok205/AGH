"""
File covering Binary Search Tree (BST) operations.
"""
from DSA import Node

class BST:

    def __init__(self):
        self.root = None

    def insert(self,val):

        def _insert(node, _val):
            if node is None:
                return Node(_val)
            if _val < node.val:
                node.left = _insert(node.left, _val)
            else:
                node.right = _insert(node.right, _val)
            return node

        self.root = _insert(self.root, val)

    def traverse(self):
        def _traverse(node):
            if node.left:
                print(node.left.val, end=" ")
                _traverse(node.left)
            if node.right:
                _traverse(node.right)
                print(node.right.val, end=" ")

        return _traverse(self.root)

    def preorder_traverse(self):
        def _preorder_traverse(node):
            if node:
                print(node.val, end=" ")
                _preorder_traverse(node.left)
                _preorder_traverse(node.right)

        _preorder_traverse(self.root)
        print("")


    def inorder_traverse(self):
        def _inorder_traverse(node):
            if node:
                _inorder_traverse(node.left)
                print(node.val, end =" ")
                _inorder_traverse(node.right)

        _inorder_traverse(self.root)
        print("")

    def postorder_traverse(self):
        def _postorder_traverse(node):
            if node:
                _postorder_traverse(node.left)
                _postorder_traverse(node.right)
                print(node.val, end =" ")
        _postorder_traverse(self.root)
        print("")

    def search(self,val):
        if self.root is None:
            return False
        current = self.root
        while current:
            if current.val == val:
                return True
            elif val < current.val:
                current = current.left
            else:
                current = current.right

        return False

    def print_min(self):
        current = self.root
        while current and current.left:
            current = current.left
        if current:
            print(f"Minimum value in BST: {current.val}")

    def print_max(self):
        current = self.root
        while current and current.right:
            current = current.right
        if current:
            print(f"Maximum value in BST: {current.val}")

    def node_successor(self):
        def _node_successor(node):
            if node.right:
                current = node.right
                while current.left:
                    current = current.left
                return current.val
            return None

        return _node_successor(self.root)


if __name__ == "__main__":
    bst = BST()
    bst.insert(5)
    bst.insert(3)
    bst.insert(2)
    bst.insert(4)
    bst.insert(7)
    bst.insert(6)
    bst.insert(8)
    bst.insert(1)
    bst.insert(9)
    bst.insert(10)
    bst.insert(0)
    bst.insert(11)


    print("BST:")
    bst.print_tree()

    print("Preorder Traversal:")
    bst.preorder_traverse()

    print("Inorder Traversal:")
    bst.inorder_traverse()

    print("Postorder Traversal:")
    bst.postorder_traverse()

    print(f"Find 5: {bst.search(5)}")  # True
    print(f"Find 12: {bst.search(12)}") # False

    bst.print_min()
    bst.print_max()

    print(f"Node Successor: {bst.node_successor()}")