//Given an array nums of distinct integers, return all the possible
//permutations. You can return the answer in any order.
//
//
// Example 1:
// Input: nums = [1,2,3]
//Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
//
// Example 2:
// Input: nums = [0,1]
//Output: [[0,1],[1,0]]
//
// Example 3:
// Input: nums = [1]
//Output: [[1]]
//
//
// Constraints:
//
//
// 1 <= nums.length <= 6
// -10 <= nums[i] <= 10
// All the integers of nums are unique.
//
//
// Related Topics Array Backtracking 👍 16329 👎 263

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::BTreeSet;

impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = BTreeSet::new();
        let len = nums.len() - 1;
        // todo!
        for _ in 0..=len {
            res.insert(nums.clone());
            for i in 0..=len {
                for j in (i + 1)..=len {
                    let mut clone = nums.clone();
                    clone.swap(i, j);
                    res.insert(clone);
                }
            }
            nums.rotate_left(1);
        }

        res.into_iter().collect()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
