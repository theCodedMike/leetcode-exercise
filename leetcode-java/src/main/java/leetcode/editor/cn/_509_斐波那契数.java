package leetcode.editor.cn;
//斐波那契数 （通常用 F(n) 表示）形成的序列称为 斐波那契数列 。该数列由 0 和 1 开始，后面的每一项数字都是前面两项数字的和。也就是： 
//
// 
//F(0) = 0，F(1) = 1
//F(n) = F(n - 1) + F(n - 2)，其中 n > 1
// 
//
// 给定 n ，请计算 F(n) 。 
//
// 
//
// 示例 1： 
//
// 
//输入：n = 2
//输出：1
//解释：F(2) = F(1) + F(0) = 1 + 0 = 1
// 
//
// 示例 2： 
//
// 
//输入：n = 3
//输出：2
//解释：F(3) = F(2) + F(1) = 1 + 1 = 2
// 
//
// 示例 3： 
//
// 
//输入：n = 4
//输出：3
//解释：F(4) = F(3) + F(2) = 2 + 1 = 3
// 
//
// 
//
// 提示： 
//
// 
// 0 <= n <= 30 
// 
//
// Related Topics 递归 记忆化搜索 数学 动态规划 👍 747 👎 0


import java.util.Arrays;
import java.util.function.BiFunction;

//leetcode submit region begin(Prohibit modification and deletion)
public class _509_斐波那契数 {
    public int fib(int n) {
        //return this.recursion(n);

        //return this.dp(n);

        //return this.matrixFastPower(n);

        return this.generalFormula(n);
    }

    BiFunction<Integer, int[], Integer> recur = (n, s) -> {
        if (s[n] != -1) {
            return s[n];
        }

        int res = this.recur.apply(n - 1, s) + this.recur.apply(n - 2, s);
        s[n] = res;

        return res;
    };

    /**
     * 时间复杂度：O(n)
     * 空间复杂度：O(n)
     */
    int recursion(int n) {
        if (n < 2) {
            return n;
        }

        int[] s = new int[n + 1];
        Arrays.fill(s, -1);
        s[0] = 0;
        s[1] = 1;

        return this.recur.apply(n, s);
    }

    /**
     * 时间复杂度：O(n)
     * 空间复杂度：O(1)
     */
    int dp(int n) {
        if (n < 2) {
            return n;
        }

        int prev = 0, curr = 0, sum = 1;
        for (int i = 1; i < n; i++) {
            prev = curr;
            curr = sum;
            sum = prev + curr;
        }

        return sum;
    }

    BiFunction<int[][], int[][], int[][]> matrixMultiply = (a, b) -> {
        int[][] c = new int[][]{{0, 0}, {0, 0}};
        for (int i = 0; i < 2; i++) {
            for (int j = 0; j < 2; j++) {
                c[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j];
            }
        }
        return c;
    };
    BiFunction<int[][], Integer, int[][]> matrixPow = (a, n) -> {
        int[][] ret = new int[][]{{1, 0}, {0, 1}};
        while (n > 0) {
            if ((n & 1) != 0) {
                ret = this.matrixMultiply.apply(ret, a);
            }
            n >>= 1;
            a = this.matrixMultiply.apply(a, a);
        }
        return ret;
    };

    /**
     * 时间复杂度：O(log(n))
     * 空间复杂度：O(1)
     */
    int matrixFastPower(int n) {
        if (n < 2) {
            return n;
        }

        int[][] m = new int[][]{{1, 1}, {1, 0}};
        int[][] res = this.matrixPow.apply(m, n - 1);

        return res[0][0];
    }

    /**
     * 时间复杂度：O(?)
     * 空间复杂度：O(1)
     */
    int generalFormula(int n) {
        double sqrt5 = Math.sqrt(5);
        double fibN = Math.pow((1 + sqrt5) / 2, n) - Math.pow((1 - sqrt5) / 2, n);
        return (int) Math.round(fibN / sqrt5);
    }
}
//leetcode submit region end(Prohibit modification and deletion)
