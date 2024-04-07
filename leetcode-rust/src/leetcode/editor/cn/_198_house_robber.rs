//You are a professional robber planning to rob houses along a street. Each
//house has a certain amount of money stashed, the only constraint stopping you from
//robbing each of them is that adjacent houses have security systems connected and
//it will automatically contact the police if two adjacent houses were broken
//into on the same night.
//
// Given an integer array nums representing the amount of money of each house,
//return the maximum amount of money you can rob tonight without alerting the
//police.
//
//
// Example 1:
//
//
//Input: nums = [1,2,3,1]
//Output: 4
//Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
//Total amount you can rob = 1 + 3 = 4.
//
//
// Example 2:
//
//
//Input: nums = [2,7,9,3,1]
//Output: 12
//Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5
//(money = 1).
//Total amount you can rob = 2 + 9 + 1 = 12.
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 100
// 0 <= nums[i] <= 400
//
//
// Related Topics Array Dynamic Programming 👍 19277 👎 359

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cmp::max;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        //Self::dp(nums)

        Self::optimize_dp(nums)
    }

    /// Time Complexity: O(n)
    /// Space Complexity: O(n)
    fn dp(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        match len {
            0 => 0,
            1 => nums[0],
            _ => {
                let mut dp = vec![0; len];
                (dp[0], dp[1]) = (nums[0], max(nums[0], nums[1]));

                for i in 2..len {
                    dp[i] = max(dp[i - 2] + nums[i], dp[i - 1]);
                }

                dp[len - 1]
            }
        }
    }

    /// Time Complexity: O(n)
    /// Space Complexity: O(1)
    fn optimize_dp(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        match len {
            0 => 0,
            1 => nums[0],
            _ => {
                let (mut first, mut second) = (nums[0], max(nums[0], nums[1]));

                for i in 2..len {
                    (first, second) = (second, max(first + nums[i], second));
                }

                second
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
