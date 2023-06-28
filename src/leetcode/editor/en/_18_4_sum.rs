//Given an array nums of n integers, return an array of all the unique
//quadruplets [nums[a], nums[b], nums[c], nums[d]] such that:
//
//
// 0 <= a, b, c, d < n
// a, b, c, and d are distinct.
// nums[a] + nums[b] + nums[c] + nums[d] == target
//
//
// You may return the answer in any order.
//
//
// Example 1:
//
//
//Input: nums = [1,0,-1,0,-2,2], target = 0
//Output: [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
//
//
// Example 2:
//
//
//Input: nums = [2,2,2,2,2], target = 8
//Output: [[2,2,2,2]]
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 200
// -10⁹ <= nums[i] <= 10⁹
// -10⁹ <= target <= 10⁹
//
//
// Related Topics Array Two Pointers Sorting 👍 9608 👎 1141

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::BTreeSet;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let len = nums.len();
        if len < 4 {
            return vec![];
        }

        let target = target as i64;
        let mut res = BTreeSet::new();
        nums.sort_unstable();

        let mut i = 0;
        let mut j = len - 1;
        while i < j {
            let mut m = i + 1;
            let mut n = j - 1;
            while m < n {
                let mut sum = 0_i64;
                sum += (nums[i] + nums[j]) as i64;
                sum += (nums[m] + nums[n]) as i64;
                if sum < target {
                    m += 1;
                } else if sum > target {
                    n -= 1;
                } else {
                    res.insert(vec![nums[i], nums[m], nums[n], nums[j]]);
                    m += 1;
                    n -= 1;
                }
            }

            if i + 3 <= j {
                j -= 1;
            } else {
                i += 1;
                j = len - 1;
            }
        }

        res.into_iter().collect()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
