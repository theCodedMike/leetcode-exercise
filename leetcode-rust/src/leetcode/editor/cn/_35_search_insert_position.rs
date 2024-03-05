//Given a sorted array of distinct integers and a target value, return the
//index if the target is found. If not, return the index where it would be if it were
//inserted in order.
//
// You must write an algorithm with O(log n) runtime complexity.
//
//
// Example 1:
//
//
//Input: nums = [1,3,5,6], target = 5
//Output: 2
//
//
// Example 2:
//
//
//Input: nums = [1,3,5,6], target = 2
//Output: 1
//
//
// Example 3:
//
//
//Input: nums = [1,3,5,6], target = 7
//Output: 4
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 10⁴
// -10⁴ <= nums[i] <= 10⁴
// nums contains distinct values sorted in ascending order.
// -10⁴ <= target <= 10⁴
//
//
// Related Topics Array Binary Search 👍 13519 👎 589

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        //Self::binary_search_1(nums, target)
        Self::binary_search_2(nums, target)
    }
    pub fn binary_search_1(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if target < nums[mid] {
                right = mid;
            } else if nums[mid] < target {
                left = mid + 1;
            } else {
                return mid as i32;
            }
        }

        left as i32
    }

    pub fn binary_search_2(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0_i32;
        let mut right = nums.len() as i32 - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            if target < nums[mid as usize] {
                right = mid - 1;
            } else if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                return mid;
            }
        }

        left
    }
}
//leetcode submit region end(Prohibit modification and deletion)
