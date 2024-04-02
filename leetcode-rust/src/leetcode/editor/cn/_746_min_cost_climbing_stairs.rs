//You are given an integer array cost where cost[i] is the cost of iᵗʰ step on
//a staircase. Once you pay the cost, you can either climb one or two steps.
//
// You can either start from the step with index 0, or the step with index 1.
//
// Return the minimum cost to reach the top of the floor.
//
//
// Example 1:
//
//
//Input: cost = [10,15,20]
//Output: 15
//Explanation: You will start at index 1.
//- Pay 15 and climb two steps to reach the top.
//The total cost is 15.
//
//
// Example 2:
//
//
//Input: cost = [1,100,1,1,1,100,1,1,100,1]
//Output: 6
//Explanation: You will start at index 0.
//- Pay 1 and climb two steps to reach index 2.
//- Pay 1 and climb two steps to reach index 4.
//- Pay 1 and climb two steps to reach index 6.
//- Pay 1 and climb one step to reach index 7.
//- Pay 1 and climb two steps to reach index 9.
//- Pay 1 and climb one step to reach the top.
//The total cost is 6.
//
//
//
// Constraints:
//
//
// 2 <= cost.length <= 1000
// 0 <= cost[i] <= 999
//
//
// Related Topics 数组 动态规划 👍 1459 👎 0

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        //Self::dp(cost)

        Self::optimize_dp(cost)
    }

    /// Time Complexity: O(n)
    /// Space Complexity: O(n)
    fn dp(cost: Vec<i32>) -> i32 {
        let len = cost.len();
        let mut dp = vec![0; len + 1];

        for i in 2..=len {
            dp[i] = std::cmp::min(dp[i - 1] + cost[i - 1], dp[i - 2] + cost[i - 2]);
        }

        dp[len]
    }

    /// Time Complexity: O(n)
    /// Space Complexity: O(1)
    fn optimize_dp(cost: Vec<i32>) -> i32 {
        let (mut prev, mut curr) = (0, 0);

        for i in 2..=cost.len() {
            let next = std::cmp::min(curr + cost[i - 1], prev + cost[i - 2]);
            (prev, curr) = (curr, next);
        }

        curr
    }
}
//leetcode submit region end(Prohibit modification and deletion)
