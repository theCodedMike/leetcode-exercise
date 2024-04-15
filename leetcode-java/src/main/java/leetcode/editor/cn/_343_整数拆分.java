package leetcode.editor.cn;
//给定一个正整数 n ，将其拆分为 k 个 正整数 的和（ k >= 2 ），并使这些整数的乘积最大化。 
//
// 返回 你可以获得的最大乘积 。 
//
// 
//
// 示例 1: 
//
// 
//输入: n = 2
//输出: 1
//解释: 2 = 1 + 1, 1 × 1 = 1。 
//
// 示例 2: 
//
// 
//输入: n = 10
//输出: 36
//解释: 10 = 3 + 3 + 4, 3 × 3 × 4 = 36。 
//
// 
//
// 提示: 
//
// 
// 2 <= n <= 58 
// 
//
// Related Topics 数学 动态规划 👍 1368 👎 0


//leetcode submit region begin(Prohibit modification and deletion)
public class _343_整数拆分 {
    public int integerBreak(int n) {
        //return this.dp(n);

        //return this.optimizeDP(n);

        return this.mathOp(n);
    }

    /**
     * 时间复杂度：O(n^2)
     * 空间复杂度：O(n)
     */
    int dp(int n) {
        int[] dp = new int[n + 1];

        for (int i = 2; i <= n; i++) {
            int currMax = 0;
            for (int j = 1; j < i; j++) {
                currMax = Math.max(currMax, Math.max(j * (i - j), j * dp[i - j]));
            }
            dp[i] = currMax;
        }

        return dp[n];
    }

    /**
     * 时间复杂度：O(n)
     * 空间复杂度：O(n)
     */
    int optimizeDP(int n) {
        if (n <= 3) {
            return n - 1;
        }

        int[] dp = new int[n + 1];
        dp[2] = 1;

        for (int i = 3; i <= n; i++) {
            dp[i] = Math.max(
                    Math.max(2 * (i - 2), 2 * dp[i - 2]),
                    Math.max(3 * (i - 3), 3 * dp[i - 3])
            );
        }

        return dp[n];
    }

    /**
     * 时间复杂度：O(1)
     * 空间复杂度：O(1)
     */
    int mathOp(int n) {
        if (n <= 3) {
            return n - 1;
        }

        int quotient = n / 3, remainder = n % 3;
        switch (remainder) {
            case 0 -> {
                return (int) Math.pow(3, quotient);
            }
            case 1 -> {
                return (int) (Math.pow(3, quotient - 1) * 4);
            }
            default -> {
                return (int) (Math.pow(3, quotient) * 2);
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
