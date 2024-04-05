package leetcode.editor.cn;
//泰波那契序列 Tn 定义如下： 
//
// T0 = 0, T1 = 1, T2 = 1, 且在 n >= 0 的条件下 Tn+3 = Tn + Tn+1 + Tn+2 
//
// 给你整数 n，请返回第 n 个泰波那契数 Tn 的值。 
//
// 
//
// 示例 1： 
//
// 输入：n = 4
//输出：4
//解释：
//T_3 = 0 + 1 + 1 = 2
//T_4 = 1 + 1 + 2 = 4
// 
//
// 示例 2： 
//
// 输入：n = 25
//输出：1389537
// 
//
// 
//
// 提示： 
//
// 
// 0 <= n <= 37 
// 答案保证是一个 32 位整数，即 answer <= 2^31 - 1。 
// 
//
// Related Topics 记忆化搜索 数学 动态规划 👍 298 👎 0


import java.util.function.BiFunction;

//leetcode submit region begin(Prohibit modification and deletion)
public class _1137_第N个泰波那契数 {
    public int tribonacci(int n) {
        return dpRecur(n);
        //return dpIter(n);

        //return matrixFastPower(n);
    }

    BiFunction<long[], Integer, Long> recur = (cache, n) -> {
        if (cache[n] != -1) {
            return cache[n];
        }

        long sum = this.recur.apply(cache, n - 1)
                + this.recur.apply(cache, n - 2)
                + this.recur.apply(cache, n - 3);
        cache[n] = sum;

        return sum;
    };

    int dpRecur(int n) {
        int len = n < 3 ? 3 : n + 1;
        long[] cache = new long[len];
        for (int i = 0; i < len; i++) {
            if (i == 0) {
                cache[i] = 0;
            } else if (i == 1 || i == 2) {
                cache[i] = 1;
            } else {
                cache[i] = -1;
            }
        }

        return this.recur.apply(cache, n).intValue();
    }

    int dpIter(int n) {
        switch (n) {
            case 0 -> {
                return 0;
            }
            case 1, 2 -> {
                return 1;
            }
            default -> {
                long t0 = 0, t1 = 0, t2 = 1, sum = 1;
                for (int i = 2; i < n; i++) {
                    t0 = t1;
                    t1 = t2;
                    t2 = sum;
                    sum = t0 + t1 + t2;
                }
                return (int) sum;
            }
        }
    }

    int matrixFastPower(int n) {
        if (n == 0) {
            return 0;
        }
        if (n <= 2) {
            return 1;
        }

        BiFunction<int[][], int[][], int[][]> multiply = (a, b) -> {
            int[][] c = new int[3][3];

            for (int i = 0; i < 3; i++) {
                for (int j = 0; j < 3; j++) {
                    c[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j];
                }
            }

            return c;
        };
        int[][] m = new int[][]{new int[]{1, 1, 1}, new int[]{1, 0, 0}, new int[]{0, 1, 0}};
        BiFunction<int[][], Integer, int[][]> pow = (_m, _n) -> {
            int[][] ret = new int[][]{new int[]{1, 0, 0}, new int[]{0, 1, 0}, new int[]{0, 0, 1}};

            while (_n > 0) {
                if ((_n & 1) == 1) {
                    ret = multiply.apply(ret, _m);
                }
                _n >>= 1;
                _m = multiply.apply(_m, _m);
            }

            return ret;
        };

        int[][] res = pow.apply(m, n);
        return res[0][2];
    }
}
//leetcode submit region end(Prohibit modification and deletion)
