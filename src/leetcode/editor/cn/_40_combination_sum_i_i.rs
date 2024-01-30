//Given a collection of candidate numbers (candidates) and a target number (
//target), find all unique combinations in candidates where the candidate numbers sum
//to target.
//
// Each number in candidates may only be used once in the combination.
//
// Note: The solution set must not contain duplicate combinations.
//
//
// Example 1:
//
//
//Input: candidates = [10,1,2,7,6,1,5], target = 8
//Output:
//[
//[1,1,6],
//[1,2,5],
//[1,7],
//[2,6]
//]
//
//
// Example 2:
//
//
//Input: candidates = [2,5,2,1,2], target = 5
//Output:
//[
//[1,2,2],
//[5]
//]
//
//
//
// Constraints:
//
//
// 1 <= candidates.length <= 100
// 1 <= candidates[i] <= 50
// 1 <= target <= 30
//
//
// Related Topics Array Backtracking 👍 8988 👎 228

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::BTreeMap;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        //Self::backtracking_1(candidates, target)

        Self::backtracking_2(candidates, target)
    }

    fn backtracking_1(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        const BACKTRACK: fn(usize, &[i32], i32, &mut Vec<i32>, &mut Vec<Vec<i32>>) =
            |idx, candidates, target, combine, res| {
                if target == 0 {
                    res.push(combine.clone());
                    return;
                }

                for i in idx..candidates.len() {
                    if i > idx && candidates[i] == candidates[i - 1] {
                        continue;
                    }
                    if target < candidates[i] {
                        break;
                    }
                    combine.push(candidates[i]);
                    BACKTRACK(i + 1, candidates, target - candidates[i], combine, res);
                    combine.pop();
                }
            };
        let mut res = vec![];

        BACKTRACK(0, &candidates, target, &mut vec![], &mut res);

        res
    }

    fn backtracking_2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let freq = candidates
            .into_iter()
            .fold(BTreeMap::new(), |mut map, val| {
                map.entry(val).and_modify(|freq| *freq += 1).or_insert(1);
                map
            })
            .into_iter()
            .collect::<Vec<_>>();
        const BACKTRACK: fn(usize, &[(i32, i32)], i32, &mut Vec<i32>, &mut Vec<Vec<i32>>) =
            |idx, freq, target, combine, res| {
                if target == 0 {
                    res.push(combine.clone());
                    return;
                }

                if idx == freq.len() || target < freq[idx].0 {
                    return;
                }

                BACKTRACK(idx + 1, freq, target, combine, res);

                let most = std::cmp::min(target / freq[idx].0, freq[idx].1);
                for i in 1..=most {
                    combine.push(freq[idx].0);
                    BACKTRACK(idx + 1, freq, target - i * freq[idx].0, combine, res);
                }

                for _ in 1..=most {
                    combine.pop();
                }
            };
        let mut res = vec![];

        BACKTRACK(0, &freq, target, &mut vec![], &mut res);

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
