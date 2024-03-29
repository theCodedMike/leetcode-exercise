//The Fibonacci numbers, commonly denoted F(n) form a sequence, called the
//Fibonacci sequence, such that each number is the sum of the two preceding ones,
//starting from 0 and 1. That is,
//
//
//F(0) = 0, F(1) = 1
//F(n) = F(n - 1) + F(n - 2), for n > 1.
//
//
// Given n, calculate F(n).
//
//
// Example 1:
//
//
//Input: n = 2
//Output: 1
//Explanation: F(2) = F(1) + F(0) = 1 + 0 = 1.
//
//
// Example 2:
//
//
//Input: n = 3
//Output: 2
//Explanation: F(3) = F(2) + F(1) = 1 + 1 = 2.
//
//
// Example 3:
//
//
//Input: n = 4
//Output: 3
//Explanation: F(4) = F(3) + F(2) = 2 + 1 = 3.
//
//
//
// Constraints:
//
//
// 0 <= n <= 30
//
//
// Related Topics 递归 记忆化搜索 数学 动态规划 👍 747 👎 0

#![allow(dead_code)]
#![allow(unused_assignments)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn fib(n: i32) -> i32 {
        Self::recursion(n)

        //Self::dp(n)

        //Self::matrix_fast_power(n)

        //Self::general_formula(n)
    }

    /// Time Complexity: O(n)
    /// Space Complexity: O(n)
    fn recursion(n: i32) -> i32 {
        if n < 2 {
            return n;
        }

        let mut s = vec![-1; n as usize + 1];
        (s[0], s[1]) = (0, 1);

        const RECUR: fn(usize, &mut [i32]) -> i32 = |n, s| {
            if s[n] != -1 {
                return s[n];
            }

            let res = RECUR(n - 1, s) + RECUR(n - 2, s);
            s[n] = res;

            res
        };

        RECUR(n as usize, &mut s)
    }

    /// Time Complexity: O(n)
    /// Space Complexity: O(1)
    fn dp(n: i32) -> i32 {
        if n < 2 {
            return n;
        }

        let (mut prev, mut curr, mut sum) = (0, 0, 1);

        for _ in 1..n {
            prev = curr;
            curr = sum;
            sum = prev + curr;
        }

        sum
    }

    /// Time Complexity: O(log(n))
    /// Space Complexity: O(1)
    fn matrix_fast_power(n: i32) -> i32 {
        if n < 2 {
            return n;
        }

        let matrix_multiply = |a: &[Vec<i32>], b: &[Vec<i32>]| {
            let mut c = vec![vec![0, 0], vec![0, 0]];
            for i in 0..2 {
                for j in 0..2 {
                    c[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j];
                }
            }
            c
        };
        let matrix_pow = |mut a: Vec<Vec<i32>>, mut n: i32| {
            let mut ret = vec![vec![1, 0], vec![0, 1]];
            while n > 0 {
                if n & 1 != 0 {
                    ret = matrix_multiply(&ret, &a);
                }
                n >>= 1;
                a = matrix_multiply(&a, &a);
            }
            ret
        };

        let m = vec![vec![1, 1], vec![1, 0]];
        let res = matrix_pow(m, n - 1);

        res[0][0]
    }

    /// Time Complexity: O(?)
    /// Space Complexity: O(1)
    fn general_formula(n: i32) -> i32 {
        let sqrt_5 = 5_f64.sqrt();
        let fib_n = ((1.0 + sqrt_5) / 2.0).powi(n) - ((1.0 - sqrt_5) / 2.0).powi(n);

        (fib_n / sqrt_5).round() as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)
