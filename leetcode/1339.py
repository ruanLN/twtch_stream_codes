# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def prefix_sum(self, root: TreeNode):
        if root is not None:
            left = self.prefix_sum(root.left)
            right = self.prefix_sum(root.right)
            return root.val + left + right
        return 0

    def find_closer_subtree(self, root: TreeNode):
        if root is not None:
            left = self.find_closer_subtree(root.left)
            right = self.find_closer_subtree(root.right)
            my_total = root.val + left[2] + right[2]
            my_current_diff = abs(my_total - self.target)
            return (root, min([left[1], my_current_diff, right[1]]), my_total)
        else:
            return (None, float('inf'), 0)
        
    def maxProduct(self, root: TreeNode) -> int:
        total = self.prefix_sum(root)
        self.target = total / 2
        result = self.find_closer_subtree(root)
        return (int)((self.target + result[1]) * (self.target - result[1])) % (10**9 + 7);

# if __name__ == "__main__":
#     app = Solution()
#     print(app.maxProduct(TreeNode(10, TreeNode(11), None)))
