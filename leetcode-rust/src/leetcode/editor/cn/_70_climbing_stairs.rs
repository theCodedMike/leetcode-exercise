//You are climbing a staircase. It takes n steps to reach the top.
//
// Each time you can either climb 1 or 2 steps. In how many distinct ways can
//you climb to the top?
//
//
// Example 1:
//
//
//Input: n = 2
//Output: 2
//Explanation: There are two ways to climb to the top.
//1. 1 step + 1 step
//2. 2 steps
//
//
// Example 2:
//
//
//Input: n = 3
//Output: 3
//Explanation: There are three ways to climb to the top.
//1. 1 step + 1 step + 1 step
//2. 1 step + 2 steps
//3. 2 steps + 1 step
//
//
//
// Constraints:
//
//
// 1 <= n <= 45
//
//
// Related Topics Math Dynamic Programming Memoization 👍 18478 👎 580

#![allow(dead_code)]
#![allow(unused_assignments)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        //Self::dp(n)

        Self::matrix_fast_power(n)

        //Self::general_formula(n)
    }

    /// Time Complexity: O(n)
    /// Space Complexity: O(1)
    fn dp(n: i32) -> i32 {
        let (mut p, mut q, mut r) = (0, 0, 1);

        for _ in 0..n {
            p = q;
            q = r;
            r = p + q;
        }

        r
    }

    /// Time Complexity: O(log(n))
    /// Space Complexity: O(1)
    fn matrix_fast_power(mut n: i32) -> i32 {
        let mut m = vec![vec![1, 1], vec![1, 0]];

        let multiply = |a: &[Vec<i64>], b: &[Vec<i64>]| {
            let mut c = vec![vec![0; 2]; 2];
            for i in 0..2 {
                for j in 0..2 {
                    c[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j];
                }
            }
            c
        };
        let mut pow = || {
            let mut ret = vec![vec![1, 0], vec![0, 1]];
            while n > 0 {
                if n & 1 == 1 {
                    ret = multiply(&ret, &m);
                }
                n >>= 1;
                m = multiply(&m, &m);
            }
            ret
        };

        let res = pow();

        res[0][0] as i32
    }

    /// Time Complexity: O(?)
    /// Space Complexity: O(1)
    fn general_formula(n: i32) -> i32 {
        let sqrt5 = 5_f64.sqrt();
        let fib_n = ((1.0 + sqrt5) / 2.0).powi(n + 1) - ((1.0 - sqrt5) / 2.0).powi(n + 1);

        return (fib_n / sqrt5).round() as i32;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
