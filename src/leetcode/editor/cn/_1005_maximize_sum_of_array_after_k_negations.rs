//Given an integer array nums and an integer k, modify the array in the
//following way:
//
//
// choose an index i and replace nums[i] with -nums[i].
//
//
// You should apply this process exactly k times. You may choose the same index
//i multiple times.
//
// Return the largest possible sum of the array after modifying it in this way.
//
//
//
// Example 1:
//
//
//Input: nums = [4,2,3], k = 1
//Output: 5
//Explanation: Choose index 1 and nums becomes [4,-2,3].
//
//
// Example 2:
//
//
//Input: nums = [3,-1,0,2], k = 3
//Output: 6
//Explanation: Choose indices (1, 2, 2) and nums becomes [3,1,0,2].
//
//
// Example 3:
//
//
//Input: nums = [2,-3,-1,5,-4], k = 2
//Output: 13
//Explanation: Choose indices (1, 4) and nums becomes [2,3,-1,5,4].
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 10⁴
// -100 <= nums[i] <= 100
// 1 <= k <= 10⁴
//
//
// Related Topics 贪心 数组 排序 👍 436 👎 0

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
        //Self::modify_neg_num(nums, k)

        //Self::sort_then_modify(nums, k)

        Self::use_min_heap(nums, k)
    }

    ///
    /// Time Complexity: O(n + C)，n is the len of nums, and C is range of the elems
    /// Space Complexity: O(C)
    ///
    fn modify_neg_num(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut freq = HashMap::with_capacity(nums.len());
        let mut sum = 0;
        for num in nums {
            sum += num;
            freq.entry(num).and_modify(|c| *c += 1).or_insert(1);
        }

        for i in -100..0 {
            if freq.contains_key(&i) {
                let ops = std::cmp::min(k, freq[&i]);
                sum += (-i) * ops * 2;
                *freq.entry(i).or_insert(0) -= ops;
                *freq.entry(-i).or_insert(0) += ops;
                k -= ops;
                if k == 0 {
                    break;
                }
            }
        }

        if k % 2 == 1 && !freq.contains_key(&0) {
            for i in 1..=100 {
                if freq.contains_key(&i) && freq[&i] != 0 {
                    sum -= i * 2;
                    break;
                }
            }
        }

        sum
    }

    ///
    /// Time Complexity: O(n * log(n))
    /// Space Complexity: O(n)
    ///
    fn sort_then_modify(mut nums: Vec<i32>, mut k: i32) -> i32 {
        nums.sort_unstable();
        let mut sum = 0;
        let mut min = i32::MAX;

        for i in 0..nums.len() {
            if nums[i] < 0 && k > 0 {
                nums[i] = -nums[i];
                k -= 1;
            }
            sum += nums[i];
            if nums[i] < min {
                min = nums[i];
            }
        }

        sum - (if k % 2 == 0 { 0 } else { 2 * min })
    }

    ///
    /// Time Complexity: O(n * log(n))
    /// Space Complexity: O(n)
    ///
    fn use_min_heap(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut heap = BinaryHeap::with_capacity(nums.len());
        let mut sum = 0;

        for num in nums {
            sum += num;
            heap.push(std::cmp::Reverse(num));
        }

        while k > 0 {
            let min = heap.pop().unwrap_or_default().0;
            sum -= 2 * min;
            heap.push(std::cmp::Reverse(-min));
            k -= 1;
        }

        sum
    }
}
//leetcode submit region end(Prohibit modification and deletion)
