//Given an unsorted array of integers nums, return the length of the longest
//consecutive elements sequence.
//
// You must write an algorithm that runs in O(n) time.
//
//
// Example 1:
//
//
//Input: nums = [100,4,200,1,3,2]
//Output: 4
//Explanation: The longest consecutive elements sequence is [1, 2, 3, 4].
//Therefore its length is 4.
//
//
// Example 2:
//
//
//Input: nums = [0,3,7,2,5,8,4,6,0,1]
//Output: 9
//
//
//
// Constraints:
//
//
// 0 <= nums.length <= 10⁵
// -10⁹ <= nums[i] <= 10⁹
//
//
// Related Topics Array Hash Table Union Find 👍 17889 👎 790

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set = nums.into_iter().collect::<HashSet<i32>>();
        let mut longest_seq = 0;

        for &num in &set {
            if !set.contains(&(num - 1)) {
                let mut curr_num = num;
                let mut curr_num_seq = 1;

                while set.contains(&(curr_num + 1)) {
                    curr_num += 1;
                    curr_num_seq += 1;
                }

                longest_seq = std::cmp::max(longest_seq, curr_num_seq);
            }
        }

        longest_seq
    }
}
//leetcode submit region end(Prohibit modification and deletion)
