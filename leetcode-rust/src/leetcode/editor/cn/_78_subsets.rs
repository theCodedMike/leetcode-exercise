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
        //Self::backtracking(nums)

        Self::iteration(nums)
    }

    fn backtracking(nums: Vec<i32>) -> Vec<Vec<i32>> {
        const DFS: fn(usize, &[i32], &mut Vec<i32>, &mut Vec<Vec<i32>>) = |i, nums, subset, res| {
            res.push(subset.clone());
            if i == nums.len() {
                return;
            }

            for j in i..nums.len() {
                subset.push(nums[j]);
                DFS(j + 1, nums, subset, res);
                subset.pop();
            }
        };
        let mut res = Vec::with_capacity(2_usize.pow(nums.len() as u32));

        DFS(0, &nums, &mut vec![], &mut res);

        res
    }

    fn iteration(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![]];

        for i in 0..nums.len() {
            for j in 0..res.len() {
                let mut subset = res[j].clone();
                subset.push(nums[i]);
                res.push(subset);
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
