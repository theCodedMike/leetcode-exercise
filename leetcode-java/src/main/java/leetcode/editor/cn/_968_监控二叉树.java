package leetcode.editor.cn;
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


import java.util.function.BiFunction;
import java.util.function.Function;

//leetcode submit region begin(Prohibit modification and deletion)
public class _968_监控二叉树 {
    public int minCameraCover(TreeNode root) {
        //return this.dp(root);

        return this.greedy(root);
    }

    Function<TreeNode, int[]> dfs1 = (root) -> {
        if (root == null) {
            return new int[]{Integer.MAX_VALUE / 2, 0, 0};
        }

        int[] left = this.dfs1.apply(root.left);
        int[] right = this.dfs1.apply(root.right);

        // 状态a：`root`必须放置摄像头的情况下，覆盖整棵树需要的摄像头数目。
        // 状态b：覆盖整棵树需要的摄像头数目，无论`root`是否放置摄像头。
        // 状态c：覆盖两棵子树需要的摄像头数目，无论节点`root`本身是否被监控到。
        int a = left[2] + right[2] + 1;
        int b = Math.min(a, Math.min(left[0] + right[1], right[0] + left[1]));
        int c = Math.min(a, left[1] + right[1]);

        return new int[]{a, b, c};
    };

    int dp(TreeNode root) {
        return this.dfs1.apply(root)[1];
    }

    BiFunction<TreeNode, int[], Integer> dfs2 = (root, res) -> {
        if (root == null) {
            return 2;
        }

        int left = this.dfs2.apply(root.left, res);
        int right = this.dfs2.apply(root.right, res);

        if (left == 2 && right == 2) {
            return 0;
        }

        if (left == 0 || right == 0) {
            res[0]++;
            return 1;
        }

        if (left == 1 || right == 1) {
            return 2;
        }

        return -1;
    };

    int greedy(TreeNode root) {
        int[] res = new int[]{0};

        // 0：该节点无覆盖
        // 1：本节点有摄像头
        // 2：本节点有覆盖
        if (this.dfs2.apply(root, res) == 0) {
            res[0]++;
        }

        return res[0];
    }
}
//leetcode submit region end(Prohibit modification and deletion)
