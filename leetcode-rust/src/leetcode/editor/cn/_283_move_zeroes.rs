//Given an integer array nums, move all 0's to the end of it while maintaining
//the relative order of the non-zero elements.
//
// Note that you must do this in-place without making a copy of the array.
//
//
// Example 1:
// Input: nums = [0,1,0,3,12]
//Output: [1,3,12,0,0]
//
// Example 2:
// Input: nums = [0]
//Output: [0]
//
//
// Constraints:
//
//
// 1 <= nums.length <= 10⁴
// -2³¹ <= nums[i] <= 2³¹ - 1
//
//
//
//Follow up: Could you minimize the total number of operations done?
//
// Related Topics Array Two Pointers 👍 15358 👎 384

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        //Self::one_pointer(nums)
        Self::double_pointers(nums)
    }

    ///  Time Complexity: O(n^2)
    ///
    /// Space Complexity: O(n)
    pub fn one_pointer(nums: &mut Vec<i32>) {
        for i in 1..nums.len() {
            if nums[i] != 0 {
                let mut j = i;
                while j > 0 && nums[j - 1] == 0 {
                    j -= 1;
                }
                nums.swap(i, j);
            }
        }
    }

    ///  Time Complexity: O(n)
    ///
    /// Space Complexity: O(1)
    pub fn double_pointers(nums: &mut Vec<i32>) {
        let mut slow = 0;

        for fast in 0..nums.len() {
            if nums[fast] != 0 {
                nums.swap(slow, fast);
                slow += 1;
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
