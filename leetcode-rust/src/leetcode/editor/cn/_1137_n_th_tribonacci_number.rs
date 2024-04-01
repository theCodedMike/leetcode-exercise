//The Tribonacci sequence Tn is defined as follows:
//
// T0 = 0, T1 = 1, T2 = 1, and Tn+3 = Tn + Tn+1 + Tn+2 for n >= 0.
//
// Given n, return the value of Tn.
//
//
// Example 1:
//
//
//Input: n = 4
//Output: 4
//Explanation:
//T_3 = 0 + 1 + 1 = 2
//T_4 = 1 + 1 + 2 = 4
//
//
// Example 2:
//
//
//Input: n = 25
//Output: 1389537
//
//
//
// Constraints:
//
//
// 0 <= n <= 37
// The answer is guaranteed to fit within a 32-bit integer, ie. answer <= 2^31 -
// 1.
//
//
// Related Topics 记忆化搜索 数学 动态规划 👍 298 👎 0

#![allow(dead_code)]
#![allow(unused_assignments)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        //Self::dp_recur(n)
        Self::dp_iter(n)

        //Self::matrix_fast_power(n)
    }

    /// Time Complexity: O(n)
    /// Space Complexity: O(n)
    fn dp_recur(n: i32) -> i32 {
        let len = if n < 3 { 3 } else { n as usize + 1 };
        let mut cache = vec![-1; len];
        (cache[0], cache[1], cache[2]) = (0, 1, 1);

        const RECUR: fn(&mut [i64], usize) -> i64 = |cache, n| {
            if cache[n] != -1 {
                return cache[n];
            }

            let sum = RECUR(cache, n - 1) + RECUR(cache, n - 2) + RECUR(cache, n - 3);
            cache[n] = sum;

            sum
        };

        RECUR(&mut cache, n as usize) as i32
    }

    /// Time Complexity: O(n)
    /// Space Complexity: O(1)
    fn dp_iter(n: i32) -> i32 {
        match n {
            0 => 0,
            1 | 2 => 1,
            _ => {
                let (mut t0, mut t1, mut t2, mut sum) = (0_i64, 0, 1, 1);

                for _ in 2..n {
                    (t0, t1, t2) = (t1, t2, sum);
                    sum = t0 + t1 + t2;
                }

                sum as i32
            }
        }
    }

    /// Time Complexity: O(log(n))
    /// Space Complexity: O(1)
    fn matrix_fast_power(mut n: i32) -> i32 {
        match n {
            0 => 0,
            1 | 2 => 1,
            _ => {
                let multiply = |a: &[Vec<i32>], b: &[Vec<i32>]| {
                    let mut c = vec![vec![0; 3]; 3];
                    for i in 0..3 {
                        for j in 0..3 {
                            c[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j];
                        }
                    }
                    c
                };
                let mut m = vec![vec![1, 1, 1], vec![1, 0, 0], vec![0, 1, 0]];
                let mut pow = || {
                    let mut ret = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
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
                res[0][2]
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
