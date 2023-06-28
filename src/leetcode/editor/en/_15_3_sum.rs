//Given an integer array nums, return all the triplets [nums[i], nums[j], nums[
//k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
//
// Notice that the solution set must not contain duplicate triplets.
//
//
// Example 1:
//
//
//Input: nums = [-1,0,1,2,-1,-4]
//Output: [[-1,-1,2],[-1,0,1]]
//Explanation:
//nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
//nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
//nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
//The distinct triplets are [-1,0,1] and [-1,-1,2].
//Notice that the order of the output and the order of the triplets does not
//matter.
//
//
// Example 2:
//
//
//Input: nums = [0,1,1]
//Output: []
//Explanation: The only possible triplet does not sum up to 0.
//
//
// Example 3:
//
//
//Input: nums = [0,0,0]
//Output: [[0,0,0]]
//Explanation: The only possible triplet sums up to 0.
//
//
//
// Constraints:
//
//
// 3 <= nums.length <= 3000
// -10⁵ <= nums[i] <= 10⁵
//
//
// Related Topics Array Two Pointers Sorting 👍 26479 👎 2382

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashSet;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = HashSet::new();
        nums.sort_unstable();

        let len = nums.len();
        for i in 0..len {
            let mut j = i + 1;
            let mut r = len - 1;
            while j < r {
                let temp = nums[i] + nums[j] + nums[r];
                if temp > 0 {
                    r -= 1;
                } else if temp < 0 {
                    j += 1;
                } else {
                    res.insert(vec![nums[i], nums[j], nums[r]]);
                    j += 1;
                    r -= 1;
                }
            }
        }

        res.into_iter().collect::<Vec<_>>()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
