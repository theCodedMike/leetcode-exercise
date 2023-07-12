//Given an unsorted integer array nums, return the smallest missing positive
//integer.
//
// You must implement an algorithm that runs in O(n) time and uses O(1)
//auxiliary space.
//
//
// Example 1:
//
//
//Input: nums = [1,2,0]
//Output: 3
//Explanation: The numbers in the range [1,2] are all in the array.
//
//
// Example 2:
//
//
//Input: nums = [3,4,-1,1]
//Output: 2
//Explanation: 1 is in the array but 2 is missing.
//
//
// Example 3:
//
//
//Input: nums = [7,8,9,11,12]
//Output: 1
//Explanation: The smallest positive integer 1 is missing.
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 10⁵
// -2³¹ <= nums[i] <= 2³¹ - 1
//
//
// Related Topics Array Hash Table 👍 14413 👎 1642

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashSet;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut the_smallest = i32::MAX;
        let mut the_largest = i32::MIN;
        let set = nums
            .into_iter()
            .map(|v| {
                if v < the_smallest {
                    the_smallest = v;
                }
                if v > the_largest {
                    the_largest = v;
                }
                v
            })
            .collect::<HashSet<_>>();

        return if the_smallest > 1 || the_largest < 1 {
            1
        } else {
            for i in 1..=the_largest {
                match set.get(&i) {
                    None => return i,
                    Some(_) => {}
                }
            }

            the_largest + 1
        };
    }
}
//leetcode submit region end(Prohibit modification and deletion)
