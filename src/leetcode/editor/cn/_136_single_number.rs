//Given a non-empty array of integers nums, every element appears twice except
//for one. Find that single one.
//
// You must implement a solution with a linear runtime complexity and use only
//constant extra space.
//
//
// Example 1:
// Input: nums = [2,2,1]
//Output: 1
//
// Example 2:
// Input: nums = [4,1,2,1,2]
//Output: 4
//
// Example 3:
// Input: nums = [1]
//Output: 1
//
//
// Constraints:
//
//
// 1 <= nums.length <= 3 * 10⁴
// -3 * 10⁴ <= nums[i] <= 3 * 10⁴
// Each element in the array appears twice except for one element which appears
//only once.
//
//
// Related Topics Array Bit Manipulation 👍 15000 👎 600

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        //Self::use_hashmap(nums)
        Self::use_bit_manipulation(nums)
    }

    pub fn use_hashmap(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for num in nums {
            map.entry(num).and_modify(|count| *count += 1).or_insert(1);
        }

        map.into_iter()
            .filter_map(|(k, v)| if v == 1 { Some(k) } else { None })
            .take(1)
            .sum()
    }

    pub fn use_bit_manipulation(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .reduce(|acc, num| acc ^ num)
            .unwrap_or_default()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
