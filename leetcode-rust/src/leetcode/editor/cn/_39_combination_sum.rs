//Given an array of distinct integers candidates and a target integer target,
//return a list of all unique combinations of candidates where the chosen numbers
//sum to target. You may return the combinations in any order.
//
// The same number may be chosen from candidates an unlimited number of times.
//Two combinations are unique if the frequency of at least one of the chosen
//numbers is different.
//
// The test cases are generated such that the number of unique combinations
//that sum up to target is less than 150 combinations for the given input.
//
//
// Example 1:
//
//
//Input: candidates = [2,3,6,7], target = 7
//Output: [[2,2,3],[7]]
//Explanation:
//2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple
//times.
//7 is a candidate, and 7 = 7.
//These are the only two combinations.
//
//
// Example 2:
//
//
//Input: candidates = [2,3,5], target = 8
//Output: [[2,2,2,2],[2,3,3],[3,5]]
//
//
// Example 3:
//
//
//Input: candidates = [2], target = 1
//Output: []
//
//
//
// Constraints:
//
//
// 1 <= candidates.length <= 30
// 2 <= candidates[i] <= 40
// All elements of candidates are distinct.
// 1 <= target <= 40
//
//
// Related Topics Array Backtracking 👍 16601 👎 334

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        //Self::backtracking_1(candidates, target)

        Self::backtracking_2(candidates, target)
    }

    fn backtracking_1(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        const BACKTRACK: fn(usize, &[i32], i32, &mut Vec<i32>, &mut Vec<Vec<i32>>) =
            |idx, candidates, target, combine, res| {
                if target < 0 {
                    return;
                }
                if target == 0 {
                    res.push(combine.clone());
                    return;
                }

                for i in idx..candidates.len() {
                    combine.push(candidates[i]);
                    BACKTRACK(i, candidates, target - candidates[i], combine, res);
                    combine.pop();
                }
            };
        let mut res = vec![];

        BACKTRACK(0, &candidates, target, &mut vec![], &mut res);

        res
    }

    fn backtracking_2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        const BACKTRACK: fn(usize, &[i32], i32, &mut Vec<i32>, &mut Vec<Vec<i32>>) =
            |idx, candidates, target, combine, res| {
                if idx == candidates.len() {
                    return;
                }
                if target == 0 {
                    res.push(combine.clone());
                    return;
                }

                BACKTRACK(idx + 1, candidates, target, combine, res);

                if target - candidates[idx] >= 0 {
                    combine.push(candidates[idx]);
                    BACKTRACK(idx, candidates, target - candidates[idx], combine, res);
                    combine.pop();
                }
            };
        let mut res = vec![];

        BACKTRACK(0, &candidates, target, &mut vec![], &mut res);

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
