//Given an integer array nums where every element appears three times except
//for one, which appears exactly once. Find the single element and return it.
//
// You must implement a solution with a linear runtime complexity and use only
//constant extra space.
//
//
// Example 1:
// Input: nums = [2,2,3,2]
//Output: 3
//
// Example 2:
// Input: nums = [0,1,0,1,0,1,99]
//Output: 99
//
//
// Constraints:
//
//
// 1 <= nums.length <= 3 * 10⁴
// -2³¹ <= nums[i] <= 2³¹ - 1
// Each element in nums appears exactly three times except for one element
//which appears once.
//
//
// Related Topics Array Bit Manipulation 👍 7285 👎 632

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        //Self::use_hashmap(nums)
        //Self::use_hashset(nums)
        //Self::use_bit_manipulation1(nums)
        Self::use_bit_manipulation2(nums)
    }

    /// 使用hashmap记录每个元素出现的次数
    ///
    /// 最后返回出现次数为1的那个元素
    pub fn use_hashmap(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::with_capacity(nums.len());
        for num in nums {
            map.entry(num).and_modify(|v| *v += 1).or_insert(1);
        }
        map.into_iter()
            .filter_map(|(k, v)| if v == 1 { Some(k) } else { None })
            .take(1)
            .sum()
    }

    /// 使用hashset去重并求和(sum1)，同时计算未去重前的总和(sum2)
    ///
    /// sum1 * n = sum2 + (n - 1) * target
    pub fn use_hashset(nums: Vec<i32>) -> i32 {
        let mut total_sum = 0_i64;
        let mut once_sum = 0_i64;
        let mut set = HashSet::new();

        for num in nums {
            let num = num as i64;
            total_sum += num;
            if !set.contains(&num) {
                once_sum += num;
                set.insert(num);
            }
        }

        ((once_sum * 3 - total_sum) / 2) as i32
    }

    /// 时间复杂度: O(nlogC)，在本题中C为32
    ///
    /// 空间复杂度: O(1)
    pub fn use_bit_manipulation1(nums: Vec<i32>) -> i32 {
        let mut res = 0;

        for i in 0..32 {
            let mut total = 0;
            for &num in &nums {
                total += (num >> i) & 1;
            }
            if total % 3 != 0 {
                res |= 1 << i;
            }
        }

        res
    }

    /// 时间复杂度: O(n)
    ///
    /// 空间复杂度: O(1)
    pub fn use_bit_manipulation2(nums: Vec<i32>) -> i32 {
        let mut a = 0;
        let mut b = 0;

        for num in nums {
            a = !b & (a ^ num);
            b = !a & (b ^ num);
        }

        a
    }
}
//leetcode submit region end(Prohibit modification and deletion)
