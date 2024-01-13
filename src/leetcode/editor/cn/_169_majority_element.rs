//Given an array nums of size n, return the majority element.
//
// The majority element is the element that appears more than ⌊n / 2⌋ times.
//You may assume that the majority element always exists in the array.
//
//
// Example 1:
// Input: nums = [3,2,3]
//Output: 3
//
// Example 2:
// Input: nums = [2,2,1,1,1,2,2]
//Output: 2
//
//
// Constraints:
//
//
// n == nums.length
// 1 <= n <= 5 * 10⁴
// -10⁹ <= nums[i] <= 10⁹
//
//
//
//Follow-up: Could you solve the problem in linear time and in
//O(1) space?
//
// Related Topics Array Hash Table Divide and Conquer Sorting Counting 👍 16875
//👎 493

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        //Self::use_hashmap(nums)
        //Self::use_sort(nums)
        //Self::use_divide_and_conquer(nums)
        Self::use_boyer_moore(nums)
    }

    /// 执行耗时:2 ms,击败了83.51% 的Rust用户   
    /// 内存消耗:2.4 MB,击败了76.32% 的Rust用户   
    /// TC: O(n)   
    /// SC: O(n)
    pub fn use_hashmap(nums: Vec<i32>) -> i32 {
        let half = nums.len() / 2;
        let mut map = HashMap::new();
        let mut maj_elem = 0;

        for num in nums {
            let v = map.entry(num).or_insert(0);
            *v += 1;
            if *v > half {
                maj_elem = num;
                break;
            }
        }

        maj_elem
    }

    /// 执行耗时:0 ms,击败了100.00% 的Rust用户   
    /// 内存消耗:2.4 MB,击败了76.32% 的Rust用户   
    /// TC: O(n*log(n))   
    /// SC: O(log(n))
    pub fn use_sort(mut nums: Vec<i32>) -> i32 {
        let half = nums.len() / 2;
        nums.sort_unstable();
        nums[half]
    }

    /// 执行耗时:958 ms,击败了8.25% 的Rust用户   
    /// 内存消耗:2.3 MB,击败了76.32% 的Rust用户   
    /// TC: O(n*log(n))   
    /// SC: O(log(n))
    pub fn use_divide_and_conquer(nums: Vec<i32>) -> i32 {
        Self::rec_divide_and_conq(&nums, 0, nums.len() - 1)
    }

    fn rec_divide_and_conq(nums: &[i32], l: usize, r: usize) -> i32 {
        // base case, the only element in an array of size 1 is the majority element
        if l == r {
            return nums[l];
        }

        // recurse on left and right halves of this slice
        let mid = (r - l) / 2 + l;
        let left = Self::rec_divide_and_conq(nums, l, mid);
        let right = Self::rec_divide_and_conq(nums, mid + 1, r);

        // if the two halves agree on the majority element, return it
        if left == right {
            return left;
        }

        // otherwise, count each element and return the "winner"
        let left_count = Self::count_in_range(nums, left, l, r);
        let right_count = Self::count_in_range(nums, right, l, r);

        if left_count > right_count {
            left
        } else {
            right
        }
    }

    fn count_in_range(nums: &[i32], num: i32, l: usize, r: usize) -> usize {
        nums.into_iter()
            .enumerate()
            .filter(|(idx, v)| l <= *idx && *idx <= r && **v == num)
            .count()
    }

    /// 执行耗时:2 ms,击败了83.51% 的Rust用户   
    /// 内存消耗:2.3 MB,击败了76.32% 的Rust用户   
    /// TC: O(log(n))
    /// SC: O(O(1))
    pub fn use_boyer_moore(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut candidate = 0;

        for num in nums {
            if count == 0 {
                candidate = num;
            }
            count += if num == candidate { 1 } else { -1 };
        }

        candidate
    }
}
//leetcode submit region end(Prohibit modification and deletion)
