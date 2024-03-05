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
        //Self::copy_array(nums)
        Self::double_pointer(nums)
    }

    ///  Time Complexity: O(n)
    ///
    /// Space Complexity: O(n)
    pub fn copy_array(nums: &mut Vec<i32>) {
        // Count the zeroes
        let mut zero_count = 0;
        for num in nums.iter() {
            if *num == 0 {
                zero_count += 1;
            }
        }

        // Make all the non-zero elements retain their original order.
        let mut copy_array = Vec::with_capacity(nums.len());
        for num in nums.iter() {
            if *num != 0 {
                copy_array.push(*num);
            }
        }

        // Move all zeroes to the end
        while zero_count != 0 {
            copy_array.push(0);
            zero_count -= 1;
        }

        // Combine the result
        for i in 0..nums.len() {
            nums[i] = copy_array[i]
        }
    }

    ///  Time Complexity: O(n)
    ///
    /// Space Complexity: O(1)
    pub fn double_pointer(nums: &mut Vec<i32>) {
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
