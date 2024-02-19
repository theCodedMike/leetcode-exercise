//Given an integer array nums, return all the different possible non-decreasing
//subsequences of the given array with at least two elements. You may return the
//answer in any order.
//
//
// Example 1:
//
//
//Input: nums = [4,6,7,7]
//Output: [[4,6],[4,6,7],[4,6,7,7],[4,7],[4,7,7],[6,7],[6,7,7],[7,7]]
//
//
// Example 2:
//
//
//Input: nums = [4,4,3,2,1]
//Output: [[4,4]]
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 15
// -100 <= nums[i] <= 100
//
//
// Related Topics 位运算 数组 哈希表 回溯 👍 771 👎 0

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashSet;
impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::backtracking(nums)
    }

    fn backtracking(nums: Vec<i32>) -> Vec<Vec<i32>> {
        const DFS: fn(usize, &Vec<i32>, &mut Vec<i32>, &mut Vec<Vec<i32>>) =
            |idx, nums, sub_seq, res| {
                if sub_seq.len() >= 2 {
                    res.push(sub_seq.clone());
                }
                if idx == nums.len() {
                    return;
                }

                let mut used = HashSet::new();
                for i in idx..nums.len() {
                    if idx > 0 && nums[idx - 1] > nums[i] {
                        continue;
                    }
                    if used.contains(&nums[i]) {
                        continue;
                    }

                    used.insert(nums[i]);
                    sub_seq.push(nums[i]);

                    DFS(i + 1, nums, sub_seq, res);

                    sub_seq.pop();
                }
            };
        let mut res = vec![];

        DFS(0, &nums, &mut vec![], &mut res);

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
