package leetcode.editor.cn;
//假设你正在爬楼梯。需要 n 阶你才能到达楼顶。 
//
// 每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？ 
//
// 
//
// 示例 1： 
//
// 
//输入：n = 2
//输出：2
//解释：有两种方法可以爬到楼顶。
//1. 1 阶 + 1 阶
//2. 2 阶 
//
// 示例 2： 
//
// 
//输入：n = 3
//输出：3
//解释：有三种方法可以爬到楼顶。
//1. 1 阶 + 1 阶 + 1 阶
//2. 1 阶 + 2 阶
//3. 2 阶 + 1 阶
// 
//
// 
//
// 提示： 
//
// 
// 1 <= n <= 45 
// 
//
// Related Topics 记忆化搜索 数学 动态规划 👍 3481 👎 0


import java.util.function.BiFunction;
import java.util.function.Function;
import java.util.function.Supplier;

//leetcode submit region begin(Prohibit modification and deletion)
class Solution {
    public int climbStairs(int n) {
        //return this.dp(n);

        //return this.matrixFastPower(n);

        //return this.generalFormula(n);

        return this.combine(n);
    }

    int dp(int n) {
        int p = 0, q = 0, r = 1;

        for (int i = 0; i < n; i++) {
            p = q;
            q = r;
            r = p + q;
        }

        return r;
    }

    int matrixFastPower(int n) {
        long[][] m = new long[][]{new long[]{1, 1}, new long[]{1, 0}};

        BiFunction<long[][], long[][], long[][]> multiply = (a, b) -> {
            long[][] c = new long[2][2];
            for (int i = 0; i < 2; i++) {
                for (int j = 0; j < 2; j++) {
                    c[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j];
                }
            }
            return c;
        };
        BiFunction<long[][], Integer, long[][]> pow = (_m, _n) -> {
            long[][] ret = new long[][]{new long[]{1, 0}, new long[]{0, 1}};

            while (_n > 0) {
                if ((_n & 1) == 1) {
                    ret = multiply.apply(ret, _m);
                }
                _n >>= 1;
                _m = multiply.apply(_m, _m);
            }
            return ret;
        };

        long[][] res = pow.apply(m, n);

        return (int) res[0][0];
    }

    int generalFormula(int n) {
        double sqrt5 = Math.sqrt(5);
        double fibN = Math.pow((1 + sqrt5) / 2, n + 1) - Math.pow((1 - sqrt5) / 2, n + 1);
        return (int) Math.round(fibN / sqrt5);
    }

    int combine(int n) {
        BiFunction<Integer, Integer, Long> calc = (a, b) -> {
            long ans = 1;
            for (int i = 1; i <= b; i++) {
                ans *= a;
                a--;
                ans /= i;
            }
            return ans;
        };

        long res = 1;
        for (int i = 1, times = n / 2; i <= times; i++) {
            res += calc.apply(n - i, i);
        }

        return (int) res;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
