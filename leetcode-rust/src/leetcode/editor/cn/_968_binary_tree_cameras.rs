//You are given the root of a binary tree. We install cameras on the tree nodes
//where each camera at a node can monitor its parent, itself, and its immediate
//children.
//
// Return the minimum number of cameras needed to monitor all nodes of the tree.
//
//
//
// Example 1:
//
//
//Input: root = [0,0,null,0,0]
//Output: 1
//Explanation: One camera is enough to monitor all nodes if placed as shown.
//
//
// Example 2:
//
//
//Input: root = [0,0,null,0,null,0,null,null,0]
//Output: 2
//Explanation: At least two cameras are needed to monitor all nodes of the tree.
// The above image shows one of the valid configurations of camera placement.
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [1, 1000].
// Node.val == 0
//
//
// Related Topics 树 深度优先搜索 动态规划 二叉树 👍 710 👎 0

#![allow(dead_code)]

pub struct Solution;
use crate::binary_tree::safe::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::cmp::min;
use std::rc::Rc;
impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        //Self::dp(root)

        Self::greedy(root)
    }

    fn dp(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        const DFS: fn(Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) = |root| match root {
            None => (i32::MAX / 2, 0, 0),
            Some(curr) => {
                let (la, lb, lc) = DFS(curr.borrow_mut().left.take());
                let (ra, rb, rc) = DFS(curr.borrow_mut().right.take());

                // 状态a：`root`必须放置摄像头的情况下，覆盖整棵树需要的摄像头数目。
                // 状态b：覆盖整棵树需要的摄像头数目，无论`root`是否放置摄像头。
                // 状态c：覆盖两棵子树需要的摄像头数目，无论节点`root`本身是否被监控到。
                let a = lc + rc + 1;
                let b = min(a, min(la + rb, ra + lb));
                let c = min(a, lb + rb);

                (a, b, c)
            }
        };

        DFS(root).1
    }

    fn greedy(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        const DFS: fn(Option<Rc<RefCell<TreeNode>>>, &mut i32) -> i32 = |root, res| match root {
            None => 2,
            Some(curr) => {
                let left = DFS(curr.borrow_mut().left.take(), res);
                let right = DFS(curr.borrow_mut().right.take(), res);

                if left == 2 && right == 2 {
                    return 0;
                }

                if left == 0 || right == 0 {
                    *res += 1;
                    return 1;
                }

                if left == 1 || right == 1 {
                    return 2;
                }

                -1
            }
        };

        let mut res = 0;
        // 0：该节点无覆盖
        // 1：本节点有摄像头
        // 2：本节点有覆盖
        if DFS(root, &mut res) == 0 {
            res += 1;
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
