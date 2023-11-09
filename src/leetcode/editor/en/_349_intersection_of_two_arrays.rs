//Given two integer arrays nums1 and nums2, return an array of their
//intersection. Each element in the result must be unique and you may return the result in
//any order.
//
//
// Example 1:
//
//
//Input: nums1 = [1,2,2,1], nums2 = [2,2]
//Output: [2]
//
//
// Example 2:
//
//
//Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
//Output: [9,4]
//Explanation: [4,9] is also accepted.
//
//
//
// Constraints:
//
//
// 1 <= nums1.length, nums2.length <= 1000
// 0 <= nums1[i], nums2[i] <= 1000
//
//
// Related Topics Array Hash Table Two Pointers Binary Search Sorting 👍 5351 👎
// 2204

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        Self::use_hash(nums1, nums2)
        //Self::sorting(nums1, nums2)
    }

    fn use_hash(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map =
            nums1
                .into_iter()
                .fold(HashMap::with_capacity(nums2.len()), |mut map, num| {
                    map.entry(num).or_insert(true);
                    map
                });

        nums2
            .into_iter()
            .filter(|num| match map.get_mut(num) {
                None => false,
                Some(val) => {
                    let contains = *val;
                    if contains {
                        *val = false;
                    }
                    contains
                }
            })
            .collect()
    }

    fn sorting(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort_unstable();
        nums2.sort_unstable();

        let mut set = HashSet::with_capacity(nums1.len());
        let mut i1 = 0;
        let mut i2 = 0;
        while i1 < nums1.len() && i2 < nums2.len() {
            if nums1[i1] < nums2[i2] {
                i1 += 1;
            } else if nums1[i1] > nums2[i2] {
                i2 += 1;
            } else {
                set.insert(nums1[i1]);
                i1 += 1;
                i2 += 1;
            }
        }

        set.into_iter().collect()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
