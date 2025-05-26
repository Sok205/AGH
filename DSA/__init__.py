class Node:
    def __init__(self,val):
        self.val = val
        self.left = None
        self.right = None
        self.height = 1

def print_tree(self):
    def _print_tree(node, level=0):
        if node is not None:
            _print_tree(node.right, level + 1)
            print(' ' * 4 * level + '->', node.val)
            _print_tree(node.left, level + 1)
    _print_tree(self.root)