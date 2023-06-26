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
// Related Topics Array Two Pointers Sorting 👍 9336 👎 484

#![allow(dead_code)]

// 执行耗时:67 ms,击败了10.27% 的Rust用户
// 内存消耗:2.8 MB,击败了12.97% 的Rust用户
pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        let mut sums = Vec::new();
        let mut res = i32::MIN;
        nums.sort_unstable();

        'outer: for i in 0..len {
            let mut j = i + 1;
            let mut k = len - 1;

            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if sum < target {
                    j += 1;
                    sums.push(sum);
                } else if sum > target {
                    k -= 1;
                    sums.push(sum);
                } else {
                    res = target;
                    break 'outer;
                }
            }
        }

        if res != target {
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
                            idx = idx - 1;
                        }
                    }

                    idx
                }
            };
            res = sums[idx];
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
