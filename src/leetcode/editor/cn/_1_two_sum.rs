//Given an array of integers nums and an integer target, return indices of the
//two numbers such that they add up to target.
//
// You may assume that each input would have exactly one solution, and you may
//not use the same element twice.
//
// You can return the answer in any order.
//
//
// Example 1:
//
//
//Input: nums = [2,7,11,15], target = 9
//Output: [0,1]
//Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
//
//
// Example 2:
//
//
//Input: nums = [3,2,4], target = 6
//Output: [1,2]
//
//
// Example 3:
//
//
//Input: nums = [3,3], target = 6
//Output: [0,1]
//
//
//
// Constraints:
//
//
// 2 <= nums.length <= 10⁴
// -10⁹ <= nums[i] <= 10⁹
// -10⁹ <= target <= 10⁹
// Only one valid answer exists.
//
//
//
//Follow-up: Can you come up with an algorithm that is less than
//O(n²)
// time complexity?
//
// Related Topics Array Hash Table 👍 51486 👎 1670

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Self::brute_force(nums, target)
        //Self::use_hash(nums, target)
    }

    fn brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len = nums.len();
        let mut res = Vec::with_capacity(2);

        'outer: for i in 0..len {
            for j in i + 1..len {
                if nums[i] + nums[j] == target {
                    res.push(i as i32);
                    res.push(j as i32);
                    break 'outer;
                }
            }
        }

        res
    }

    fn use_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len = nums.len();
        let mut map = HashMap::with_capacity(len);
        let mut res = Vec::with_capacity(2);

        for i in 0..len {
            let diff = target - nums[i];
            match map.get(&diff) {
                None => {
                    map.insert(nums[i], i);
                }
                Some(idx) => {
                    res.push(*idx as i32);
                    res.push(i as i32);
                    break;
                }
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
