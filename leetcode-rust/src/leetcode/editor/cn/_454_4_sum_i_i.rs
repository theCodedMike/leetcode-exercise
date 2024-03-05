//Given four integer arrays nums1, nums2, nums3, and nums4 all of length n,
//return the number of tuples (i, j, k, l) such that:
//
//
// 0 <= i, j, k, l < n
// nums1[i] + nums2[j] + nums3[k] + nums4[l] == 0
//
//
//
// Example 1:
//
//
//Input: nums1 = [1,2], nums2 = [-2,-1], nums3 = [-1,2], nums4 = [0,2]
//Output: 2
//Explanation:
//The two tuples are:
//1. (0, 0, 0, 1) -> nums1[0] + nums2[0] + nums3[0] + nums4[1] = 1 + (-2) + (-1)
// + 2 = 0
//2. (1, 1, 0, 0) -> nums1[1] + nums2[1] + nums3[0] + nums4[0] = 2 + (-1) + (-1)
// + 0 = 0
//
//
// Example 2:
//
//
//Input: nums1 = [0], nums2 = [0], nums3 = [0], nums4 = [0]
//Output: 1
//
//
//
// Constraints:
//
//
// n == nums1.length
// n == nums2.length
// n == nums3.length
// n == nums4.length
// 1 <= n <= 200
// -2²⁸ <= nums1[i], nums2[i], nums3[i], nums4[i] <= 2²⁸
//
//
// Related Topics Array Hash Table 👍 4794 👎 138

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;
impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        Self::use_hash(nums1, nums2, nums3, nums4)
    }

    /// Time Complexity: O(n^2)
    ///
    /// Space Complexity: O(n^2)
    fn use_hash(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
        let len = nums1.len();
        let mut map = HashMap::with_capacity(len * len);
        for i in 0..len {
            for j in 0..len {
                map.entry(nums1[i] + nums2[j])
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        }

        let mut res = 0;
        for k in 0..len {
            for l in 0..len {
                if let Some(v) = map.get(&(0 - nums3[k] - nums4[l])) {
                    res += *v;
                }
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
