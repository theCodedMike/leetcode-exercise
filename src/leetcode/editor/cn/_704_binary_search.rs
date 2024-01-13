//Given an array of integers nums which is sorted in ascending order, and an
//integer target, write a function to search target in nums. If target exists, then
//return its index. Otherwise, return -1.
//
// You must write an algorithm with O(log n) runtime complexity.
//
//
// Example 1:
//
//
//Input: nums = [-1,0,3,5,9,12], target = 9
//Output: 4
//Explanation: 9 exists in nums and its index is 4
//
//
// Example 2:
//
//
//Input: nums = [-1,0,3,5,9,12], target = 2
//Output: -1
//Explanation: 2 does not exist in nums so return -1
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 10⁴
// -10⁴ < nums[i], target < 10⁴
// All the integers in nums are unique.
// nums is sorted in ascending order.
//
//
// Related Topics Array Binary Search 👍 10916 👎 217

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
macro_rules! mid_idx {
    ($left:expr, $right:expr) => {
        $left + (($right - $left) >> 1)
    };
}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        //Self::find_the_exact_value(nums, target)
        //Self::find_upper_bound(nums, target)
        //Self::find_lower_bound(nums, target)
        Self::use_built_in_tools(nums, target)
    }

    pub fn find_the_exact_value(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = mid_idx!(left, right);
            if target < nums[mid] {
                right = mid;
            } else if nums[mid] < target {
                left = mid + 1;
            } else {
                return mid as i32;
            }
        }

        -1
    }

    pub fn find_upper_bound(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = mid_idx!(left, right);
            if nums[mid] <= target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        if left > 0 && nums[left - 1] == target {
            left as i32 - 1
        } else {
            -1
        }
    }

    pub fn find_lower_bound(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = mid_idx!(left, right);
            if target <= nums[mid] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        if left < nums.len() && nums[left] == target {
            left as i32
        } else {
            -1
        }
    }

    pub fn use_built_in_tools(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(idx) => idx as i32,
            Err(_) => -1,
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
