/**
 * Definition for a binary tree node.
 * class TreeNode {
 *     val: number
 *     left: TreeNode | null
 *     right: TreeNode | null
 *     constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.left = (left===undefined ? null : left)
 *         this.right = (right===undefined ? null : right)
 *     }
 * }
 */

function sum(root: TreeNode | null): number {
    if (root) {
        let left = sum(root.left);
        let right = sum(root.right)
        return (root.val + left + right);
    } else {
        return 0;
    }
};

function findCloserSubtree(root: TreeNode | null, target: number) : [TreeNode | null, number, number] {
    if (root) {
        let left_best = findCloserSubtree(root.left, target);
        let right_best = findCloserSubtree(root.right, target);
        let my_sum = root.val + left_best[2] + right_best[2];
        //change the values to send the best to return
        left_best[2] = my_sum;
        right_best[2] = my_sum;
        let my_current : [TreeNode | null, number, number] = [root, Math.abs(my_sum - target), my_sum];
        if (left_best[1] <= right_best[1] && left_best[1] <= my_current[1]) {
            return left_best;
        } else if (right_best[1] <= left_best[1] && right_best[1] <= my_current[1]) {
            return right_best;
        } else {
            return my_current;
        }
    } else {
        return [null, Number.MAX_SAFE_INTEGER, 0];
    }
};

// achar subarvore com valor mais próximo da metede do valor total da arvore
function maxProduct(root: TreeNode | null): number {
    let total = sum(root);
    let half = total / 2;
    let closer = findCloserSubtree(root, half);    
    let total_closer = sum(closer[0]);
    let res = (total_closer * (total - total_closer)) % (10**9 + 7);
    return res;
};
