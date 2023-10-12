//Given an array of positive integers nums and a positive integer target,
//return the minimal length of a subarray whose sum is greater than or equal to target.
//If there is no such subarray, return 0 instead.
//
//
// Example 1:
//
//
//Input: target = 7, nums = [2,3,1,2,4,3]
//Output: 2
//Explanation: The subarray [4,3] has the minimal length under the problem
//constraint.
//
//
// Example 2:
//
//
//Input: target = 4, nums = [1,4,4]
//Output: 1
//
//
// Example 3:
//
//
//Input: target = 11, nums = [1,1,1,1,1,1,1,1]
//Output: 0
//
//
//
// Constraints:
//
//
// 1 <= target <= 10⁹
// 1 <= nums.length <= 10⁵
// 1 <= nums[i] <= 10⁴
//
//
//
//Follow up: If you have figured out the
//O(n) solution, try coding another solution of which the time complexity is
//O(n log(n)).
//
// Related Topics Array Binary Search Sliding Window Prefix Sum 👍 11768 👎 353

#![allow(dead_code)]

use std::usize;

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        //Self::brute_force(target, nums)
        //Self::using_binary_search(target, nums)
        Self::two_pointers(target, nums)
    }

    /// Time Limit Exceeded
    ///
    /// Time Complexity: O(n^3), Space complexity: O(1)
    pub fn brute_force(target: i32, nums: Vec<i32>) -> i32 {
        let len = nums.len();

        for width in 1..=len {
            let mut begin = 0;
            let mut end = begin + width;

            while end <= len {
                let mut sum = 0;
                for i in begin..end {
                    sum += nums[i];
                }
                if sum >= target {
                    return width as i32;
                }

                begin += 1;
                end = begin + width;
            }
        }

        0
    }

    /// Time complexity: O(nlog(n)), Space complexity: O(n)
    pub fn using_binary_search(target: i32, nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut res = usize::MAX;
        let mut sums = vec![0; len + 1];

        for i in 0..len {
            sums[i + 1] = sums[i] + nums[i];
        }
        for i in 0..len {
            let to_find = target + sums[i];
            let idx = match sums.binary_search(&to_find) {
                Ok(idx) => idx,
                Err(idx) => idx,
            };
            if idx != sums.len() {
                res = std::cmp::min(res, idx - i);
            }
        }

        if res != usize::MAX {
            res as i32
        } else {
            0
        }
    }

    /// Time complexity: O(n), Space complexity: O(1)
    pub fn two_pointers(target: i32, nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut res = usize::MAX;
        let mut left = 0;
        let mut sum = 0;

        for i in 0..len {
            sum += nums[i];

            while sum >= target {
                res = std::cmp::min(res, i + 1 - left);
                sum -= nums[left];
                left += 1;
            }
        }

        if res == usize::MAX {
            0
        } else {
            res as i32
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
