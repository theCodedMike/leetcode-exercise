//Given an array nums with n objects colored red, white, or blue, sort them in-
//place so that objects of the same color are adjacent, with the colors in the
//order red, white, and blue.
//
// We will use the integers 0, 1, and 2 to represent the color red, white, and
//blue, respectively.
//
// You must solve this problem without using the library's sort function.
//
//
// Example 1:
//
//
//Input: nums = [2,0,2,1,1,0]
//Output: [0,0,1,1,2,2]
//
//
// Example 2:
//
//
//Input: nums = [2,0,1]
//Output: [0,1,2]
//
//
//
// Constraints:
//
//
// n == nums.length
// 1 <= n <= 300
// nums[i] is either 0, 1, or 2.
//
//
//
// Follow up: Could you come up with a one-pass algorithm using only constant
//extra space?
//
// Related Topics Array Two Pointers Sorting 👍 15977 👎 560

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut counter = [0_u16; 3];
        for num in nums.iter() {
            counter[*num as usize] += 1;
        }

        let mut idx = 0;
        for i in 0..counter.len() {
            for _ in 0..counter[i] {
                nums[idx] = i as i32;
                idx += 1;
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
