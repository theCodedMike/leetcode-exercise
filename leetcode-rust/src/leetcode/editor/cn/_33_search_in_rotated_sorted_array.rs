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
        //Self::linear_search_then_binary(nums, target)
        Self::binary_search(nums, target)
    }

    ///
    /// 先线性搜索再二分搜索
    ///
    /// 执行耗时:1 ms,击败了74.12% 的Rust用户
    /// 内存消耗:2.1 MB,击败了78.52% 的Rust用户
    ///
    fn linear_search_then_binary(nums: Vec<i32>, target: i32) -> i32 {
        let mut l_idx = 0;
        let mut r_idx = nums.len();

        while l_idx < r_idx {
            if nums[l_idx] > nums[r_idx - 1] {
                if nums[r_idx - 1] == target {
                    return r_idx as i32 - 1;
                }
                r_idx -= 1;
            } else {
                let m_idx = (l_idx + r_idx) / 2;
                if target > nums[m_idx] {
                    l_idx = m_idx + 1;
                } else if target < nums[m_idx] {
                    r_idx = m_idx;
                } else {
                    return m_idx as i32;
                }
            }
        }

        -1
    }

    ///
    /// 二分搜索
    ///
    /// 执行耗时:1 ms,击败了74.12% 的Rust用户
    /// 内存消耗:2.1 MB,击败了78.52% 的Rust用户
    ///
    fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        let (mut left, mut right) = (0, len);

        while left < right {
            let mid = (left + right) >> 1;
            if nums[mid] == target {
                return mid as i32;
            }

            if nums[left] < nums[mid] {
                if nums[left] <= target && target <= nums[mid] {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            } else {
                if nums[mid] <= target && target <= nums[len - 1] {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
        }

        -1
    }
}
//leetcode submit region end(Prohibit modification and deletion)
