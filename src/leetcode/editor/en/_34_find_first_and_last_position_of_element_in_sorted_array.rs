//Given an array of integers nums sorted in non-decreasing order, find the
//starting and ending position of a given target value.
//
// If target is not found in the array, return [-1, -1].
//
// You must write an algorithm with O(log n) runtime complexity.
//
//
// Example 1:
// Input: nums = [5,7,7,8,8,10], target = 8
//Output: [3,4]
//
// Example 2:
// Input: nums = [5,7,7,8,8,10], target = 6
//Output: [-1,-1]
//
// Example 3:
// Input: nums = [], target = 0
//Output: [-1,-1]
//
//
// Constraints:
//
//
// 0 <= nums.length <= 10⁵
// -10⁹ <= nums[i] <= 10⁹
// nums is a non-decreasing array.
// -10⁹ <= target <= 10⁹
//
//
// Related Topics Array Binary Search 👍 17543 👎 429

#![allow(dead_code)]

pub struct Solution;
// 测试用例: [1]       target: 0
// 测试结果: [-1,-1]
// 测试用例: [1]       target: 1
// 测试结果: [0,0]
// 测试用例: [2,2]     target: 2
// 测试结果: [0,1]
// 测试用例: [1,3]     target: 1
// 测试结果: [0,0]

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res = vec![-1, -1];
        let len = nums.len();
        let mut l_idx = 0;
        let mut r_idx = len;

        while l_idx < r_idx {
            let m_idx = (l_idx + r_idx) / 2;
            if target < nums[m_idx] {
                r_idx = m_idx;
            } else if target > nums[m_idx] {
                l_idx = m_idx + 1;
            } else {
                l_idx = m_idx;
                r_idx = m_idx;
                loop {
                    if l_idx > 0 && nums[l_idx - 1] == target {
                        l_idx -= 1;
                    }
                    if r_idx < len - 1 && nums[r_idx + 1] == target {
                        r_idx += 1;
                    }
                    if (l_idx <= 0 || nums[l_idx - 1] != target)
                        && (r_idx >= len - 1 || nums[r_idx + 1] != target)
                    {
                        break;
                    }
                }
                res[0] = l_idx as i32;
                res[1] = r_idx as i32;
                break;
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
