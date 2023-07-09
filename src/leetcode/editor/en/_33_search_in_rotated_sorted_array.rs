//There is an integer array nums sorted in ascending order (with distinct
//values).
//
// Prior to being passed to your function, nums is possibly rotated at an
//unknown pivot index k (1 <= k < nums.length) such that the resulting array is [nums[k]
//, nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For
//example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 and become [4,5,6,7,0
//,1,2].
//
// Given the array nums after the possible rotation and an integer target,
//return the index of target if it is in nums, or -1 if it is not in nums.
//
// You must write an algorithm with O(log n) runtime complexity.
//
//
// Example 1:
// Input: nums = [4,5,6,7,0,1,2], target = 0
//Output: 4
//
// Example 2:
// Input: nums = [4,5,6,7,0,1,2], target = 3
//Output: -1
//
// Example 3:
// Input: nums = [1], target = 0
//Output: -1
//
//
// Constraints:
//
//
// 1 <= nums.length <= 5000
// -10⁴ <= nums[i] <= 10⁴
// All values of nums are unique.
// nums is an ascending array that is possibly rotated.
// -10⁴ <= target <= 10⁴
//
//
// Related Topics Array Binary Search 👍 22099 👎 1305

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l_idx = 0;
        let mut r_idx = (nums.len() - 1) as i32;

        while l_idx <= r_idx {
            if nums[l_idx as usize] > nums[r_idx as usize] {
                if nums[r_idx as usize] == target {
                    return r_idx;
                }
                r_idx -= 1;
            } else {
                let m_idx = (l_idx + r_idx) / 2;
                if target > nums[m_idx as usize] {
                    l_idx = m_idx + 1;
                } else if target < nums[m_idx as usize] {
                    r_idx = m_idx - 1;
                } else {
                    return m_idx;
                }
            }
        }

        -1
    }
}
//leetcode submit region end(Prohibit modification and deletion)
