//There is a robot on an m x n grid. The robot is initially located at the top-
//left corner (i.e., grid[0][0]). The robot tries to move to the bottom-right
//corner (i.e., grid[m - 1][n - 1]). The robot can only move either down or right at
//any point in time.
//
// Given the two integers m and n, return the number of possible unique paths
//that the robot can take to reach the bottom-right corner.
//
// The test cases are generated so that the answer will be less than or equal
//to 2 * 10⁹.
//
//
// Example 1:
//
//
//Input: m = 3, n = 7
//Output: 28
//
//
// Example 2:
//
//
//Input: m = 3, n = 2
//Output: 3
//Explanation: From the top-left corner, there are a total of 3 ways to reach
//the bottom-right corner:
//1. Right -> Down -> Down
//2. Down -> Down -> Right
//3. Down -> Right -> Down
//
//
//
// Constraints:
//
//
// 1 <= m, n <= 100
//
//
// Related Topics Math Dynamic Programming Combinatorics 👍 14485 👎 397

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        //Self::dp(m, n)

        //Self::optimize_dp(m, n)

        Self::combinatorics(m, n)
    }

    /// Time Complexity: O(m * n)
    /// Space Complexity: O(m * n)
    fn dp(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![0; n]; m];

        for i in 0..m {
            dp[i][0] = 1;
        }
        for i in 0..n {
            dp[0][i] = 1;
        }
        for i in 1..m {
            for j in 1..n {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }

        dp[m - 1][n - 1]
    }

    /// Time Complexity: O(m * n)
    /// Space Complexity: O(n)
    fn optimize_dp(m: i32, n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![1; n];

        for _ in 1..m {
            for j in 1..n {
                dp[j] += dp[j - 1];
            }
        }

        dp[n - 1]
    }

    /// Time Complexity: O(m)
    /// Space Complexity: O(1)
    fn combinatorics(m: i32, n: i32) -> i32 {
        let m = m as i64;
        let mut res = 1_i64;
        let (mut x, mut y) = (n as i64, 1);

        while y < m {
            res = res * x / y;
            (x, y) = (x + 1, y + 1);
        }

        res as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)
