//Given an integer n, break it into the sum of k positive integers, where k >= 2
//, and maximize the product of those integers.
//
// Return the maximum product you can get.
//
//
// Example 1:
//
//
//Input: n = 2
//Output: 1
//Explanation: 2 = 1 + 1, 1 × 1 = 1.
//
//
// Example 2:
//
//
//Input: n = 10
//Output: 36
//Explanation: 10 = 3 + 3 + 4, 3 × 3 × 4 = 36.
//
//
//
// Constraints:
//
//
// 2 <= n <= 58
//
//
// Related Topics 数学 动态规划 👍 1368 👎 0

#![allow(dead_code)]
pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cmp::max;
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        //Self::dp(n)

        //Self::optimize_dp(n)

        Self::math(n)
    }

    /// Time Complexity: O(n^2)
    /// Space Complexity: O(n)
    fn dp(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];

        for i in 2..=n {
            let mut curr_max = 0;
            for j in 1..i {
                curr_max = max(curr_max, max(j * (i - j), j * dp[i - j]));
            }
            dp[i] = curr_max;
        }

        dp[n] as i32
    }

    /// Time Complexity: O(n)
    /// Space Complexity: O(n)
    fn optimize_dp(n: i32) -> i32 {
        if n <= 3 {
            return n - 1;
        }

        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[2] = 1;

        for i in 3..=n {
            dp[i] = max(
                max(2 * (i - 2), 2 * dp[i - 2]),
                max(3 * (i - 3), 3 * dp[i - 3]),
            )
        }

        dp[n] as i32
    }

    /// Time Complexity: O(1)
    /// Space Complexity: O(1)
    fn math(n: i32) -> i32 {
        if n <= 3 {
            return n - 1;
        }

        let (quotient, remainder) = ((n / 3) as u32, n % 3);
        match remainder {
            0 => 3_i32.pow(quotient),
            1 => 3_i32.pow(quotient - 1) * 4,
            _ => 3_i32.pow(quotient) * 2,
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
