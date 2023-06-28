//Given an integer array nums of length n and an integer target, find three
//integers in nums such that the sum is closest to target.
//
// Return the sum of the three integers.
//
// You may assume that each input would have exactly one solution.
//
//
// Example 1:
//
//
//Input: nums = [-1,2,1,-4], target = 1
//Output: 2
//Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
//
//
// Example 2:
//
//
//Input: nums = [0,0,0], target = 1
//Output: 0
//Explanation: The sum that is closest to the target is 0. (0 + 0 + 0 = 0).
//
//
//
// Constraints:
//
//
// 3 <= nums.length <= 500
// -1000 <= nums[i] <= 1000
// -10⁴ <= target <= 10⁴
//
//
// Related Topics Array Two Pointers Sorting 👍 9352 👎 484

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        let mut sums = Vec::new();

        for i in 0..len {
            for j in i + 1..len {
                for k in j + 1..len {
                    sums.push(nums[i] + nums[j] + nums[k]);
                }
            }
        }

        sums.sort_unstable();
        let idx = match sums.binary_search(&target) {
            Ok(idx) => idx,
            Err(mut idx) => {
                if idx == sums.len() {
                    idx -= 1;
                } else if idx == 0 {
                    idx = 0;
                } else {
                    let diff_left = target - sums[idx - 1];
                    let diff_right = sums[idx] - target;

                    if diff_left <= diff_right {
                        idx -= 1;
                    }
                }

                idx
            }
        };

        sums[idx]
    }
}
//leetcode submit region end(Prohibit modification and deletion)
