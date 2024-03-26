//给定一个二叉树，我们在树的节点上安装摄像头。
//
// 节点上的每个摄影头都可以监视其父对象、自身及其直接子对象。
//
// 计算监控树的所有节点所需的最小摄像头数量。
//
//
//
// 示例 1：
//
//
//
// 输入：[0,0,null,0,0]
//输出：1
//解释：如图所示，一台摄像头足以监控所有节点。
//
//
// 示例 2：
//
//
//
// 输入：[0,0,null,0,null,0,null,null,0]
//输出：2
//解释：需要至少两个摄像头来监视树的所有节点。 上图显示了摄像头放置的有效位置之一。
//
//
// 提示：
//
//
// 给定树的节点数的范围是 [1, 1000]。
// 每个节点的值都是 0。
//
//
// Related Topics 树 深度优先搜索 动态规划 二叉树 👍 710 👎 0

package src

import "math"

// leetcode submit region begin(Prohibit modification and deletion)
func minCameraCover(root *TreeNode) int {
	//return dp(root)

	return greedy(root)
}

func dp(root *TreeNode) int {
	var dfs func(*TreeNode) (int, int, int)
	dfs = func(root *TreeNode) (int, int, int) {
		if root == nil {
			return math.MaxInt / 2, 0, 0
		}

		la, lb, lc := dfs(root.Left)
		ra, rb, rc := dfs(root.Right)

		// 状态a：`root`必须放置摄像头的情况下，覆盖整棵树需要的摄像头数目。
		// 状态b：覆盖整棵树需要的摄像头数目，无论`root`是否放置摄像头。
		// 状态c：覆盖两棵子树需要的摄像头数目，无论节点`root`本身是否被监控到。
		a := lc + rc + 1
		b := min(a, min(la+rb, ra+lb))
		c := min(a, lb+rb)

		return a, b, c
	}

	_, b, _ := dfs(root)

	return b
}

func greedy(root *TreeNode) int {
	var dfs func(*TreeNode, *int) int
	dfs = func(root *TreeNode, res *int) int {
		if root == nil {
			return 2
		}

		left := dfs(root.Left, res)
		right := dfs(root.Right, res)

		if left == 2 && right == 2 {
			return 0
		}

		if left == 0 || right == 0 {
			*res++
			return 1
		}

		if left == 1 || right == 1 {
			return 2
		}

		return -1
	}

	res := 0
	// 0：该节点无覆盖
	// 1：本节点有摄像头
	// 2：本节点有覆盖
	if dfs(root, &res) == 0 {
		res++
	}

	return res
}

//leetcode submit region end(Prohibit modification and deletion)
