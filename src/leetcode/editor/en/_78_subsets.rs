//Given an integer array nums of unique elements, return all possible subsets (
//the power set).
//
// The solution set must not contain duplicate subsets. Return the solution in
//any order.
//
//
// Example 1:
//
//
//Input: nums = [1,2,3]
//Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
//
//
// Example 2:
//
//
//Input: nums = [0]
//Output: [[],[0]]
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 10
// -10 <= nums[i] <= 10
// All the numbers of nums are unique.
//
//
// Related Topics Array Backtracking Bit Manipulation 👍 15303 👎 223

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // todo!
        let len = nums.len();
        let cap = 2_usize.pow(len as u32);
        let vec = Vec::with_capacity(cap);
        vec
    }
}
//leetcode submit region end(Prohibit modification and deletion)
