//Given an integer array nums that may contain duplicates, return all possible
//subsets (the power set).
//
// The solution set must not contain duplicate subsets. Return the solution in
//any order.
//
//
// Example 1:
// Input: nums = [1,2,2]
//Output: [[],[1],[1,2],[1,2,2],[2],[2,2]]
//
// Example 2:
// Input: nums = [0]
//Output: [[],[0]]
//
//
// Constraints:
//
//
// 1 <= nums.length <= 10
// -10 <= nums[i] <= 10
//
//
// Related Topics Array Backtracking Bit Manipulation 👍 8737 👎 246

#![allow(dead_code)]
#![allow(unused_assignments)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        //Self::backtracking(nums)

        Self::iteration(nums)
    }

    fn backtracking(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        const DFS: fn(usize, &[i32], &mut Vec<i32>, &mut Vec<Vec<i32>>) = |i, nums, subset, res| {
            res.push(subset.clone());

            if i == nums.len() {
                return;
            }

            for j in i..nums.len() {
                if j > i && nums[j] == nums[j - 1] {
                    continue;
                }
                subset.push(nums[j]);
                DFS(j + 1, nums, subset, res);
                subset.pop();
            }
        };
        let mut res = vec![];

        DFS(0, &nums, &mut vec![], &mut res);

        res
    }

    fn iteration(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut res = vec![vec![]];
        let (mut pre_len, mut len) = (0, 0);

        for i in 0..nums.len() {
            (pre_len, len) = (len, res.len());

            if i > 0 && nums[i] != nums[i - 1] {
                pre_len = 0;
            }
            for j in pre_len..len {
                let mut subset = res[j].clone();
                subset.push(nums[i]);
                res.push(subset);
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
