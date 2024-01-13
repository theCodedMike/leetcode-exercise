//Suppose an array of length n sorted in ascending order is rotated between 1
//and n times. For example, the array nums = [0,1,4,4,5,6,7] might become:
//
//
// [4,5,6,7,0,1,4] if it was rotated 4 times.
// [0,1,4,4,5,6,7] if it was rotated 7 times.
//
//
// Notice that rotating an array [a[0], a[1], a[2], ..., a[n-1]] 1 time results
//in the array [a[n-1], a[0], a[1], a[2], ..., a[n-2]].
//
// Given the sorted rotated array nums that may contain duplicates, return the
//minimum element of this array.
//
// You must decrease the overall operation steps as much as possible.
//
//
// Example 1:
// Input: nums = [1,3,5]
//Output: 1
//
// Example 2:
// Input: nums = [2,2,2,0,1]
//Output: 0
//
//
// Constraints:
//
//
// n == nums.length
// 1 <= n <= 5000
// -5000 <= nums[i] <= 5000
// nums is sorted and rotated between 1 and n times.
//
//
//
// Follow up: This problem is similar to Find Minimum in Rotated Sorted Array,
//but nums may contain duplicates. Would this affect the runtime complexity? How
//and why?
//
//
//
// Related Topics Array Binary Search 👍 4374 👎 452

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::ops::Index;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut left = 0;
        let mut right = len;

        while left < right {
            let mid = (left + right) / 2;
            if nums[left] < nums[mid] {
                left = mid;
            } else if nums[left] > nums[mid] {
                right = mid;
            } else {
                if left == mid {
                    break;
                }
                if nums.index(left..=mid).iter().all(|v| *v == nums[left]) {
                    left = mid;
                } else {
                    right = mid;
                }
            }
        }

        nums[(left + 1) % len]
    }
}
//leetcode submit region end(Prohibit modification and deletion)
