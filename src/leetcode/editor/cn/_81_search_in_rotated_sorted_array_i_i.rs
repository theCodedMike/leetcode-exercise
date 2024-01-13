//There is an integer array nums sorted in non-decreasing order (not
//necessarily with distinct values).
//
// Before being passed to your function, nums is rotated at an unknown pivot
//index k (0 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1]
//, ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0
//,1,2,4,4,4,5,6,6,7] might be rotated at pivot index 5 and become [4,5,6,6,7,0,1,
//2,4,4].
//
// Given the array nums after the rotation and an integer target, return true
//if target is in nums, or false if it is not in nums.
//
// You must decrease the overall operation steps as much as possible.
//
//
// Example 1:
// Input: nums = [2,5,6,0,0,1,2], target = 0
//Output: true
//
// Example 2:
// Input: nums = [2,5,6,0,0,1,2], target = 3
//Output: false
//
//
// Constraints:
//
//
// 1 <= nums.length <= 5000
// -10⁴ <= nums[i] <= 10⁴
// nums is guaranteed to be rotated at some pivot.
// -10⁴ <= target <= 10⁴
//
//
//
// Follow up: This problem is similar to Search in Rotated Sorted Array, but
//nums may contain duplicates. Would this affect the runtime complexity? How and why?
//
//
// Related Topics Array Binary Search 👍 6610 👎 865

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums[0] == target {
            return true;
        }

        let len = nums.len();
        let mut k = 1; // pivot index

        // find pivot index k
        for i in 1..len {
            if nums[i - 1] <= nums[i] {
                k += 1;
                if nums[i] == target {
                    return true;
                }
            } else {
                break;
            }
        }

        // binary search
        let mut left = k;
        let mut right = len;
        while left < right {
            let mid = (left + right) / 2;
            if target < nums[mid] {
                right = mid;
            } else if target > nums[mid] {
                left = mid + 1;
            } else {
                return true;
            }
        }

        false
    }
}
//leetcode submit region end(Prohibit modification and deletion)
