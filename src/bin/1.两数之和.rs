/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */
// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut result = Vec::new();

        let mut diff;
        for (idx, val) in nums.iter().enumerate() {
            diff = target - val;
            if map.contains_key(&diff) {
                result.push(*map.get(&diff).unwrap() as i32);
                result.push(idx as i32);
                break;
            } else {
                map.insert(val, idx);
            }
        }

        result
    }
}
// @lc code=end

