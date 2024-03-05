//Given two sorted arrays nums1 and nums2 of size m and n respectively, return
//the median of the two sorted arrays.
//
// The overall run time complexity should be O(log (m+n)).
//
//
// Example 1:
//
//
//Input: nums1 = [1,3], nums2 = [2]
//Output: 2.00000
//Explanation: merged array = [1,2,3] and median is 2.
//
//
// Example 2:
//
//
//Input: nums1 = [1,2], nums2 = [3,4]
//Output: 2.50000
//Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
//
//
//
// Constraints:
//
//
// nums1.length == m
// nums2.length == n
// 0 <= m <= 1000
// 0 <= n <= 1000
// 1 <= m + n <= 2000
// -10⁶ <= nums1[i], nums2[i] <= 10⁶
//
//
// Related Topics Array Binary Search Divide and Conquer 👍 23952 👎 2674

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len1 = nums1.len();
        let len2 = nums2.len();
        let len_sum = len1 + len2;
        let target_idx = len_sum >> 1;
        let mut res = [0; 2];
        let mut switch_idx = 0;

        let mut i = 0;
        let mut j = 0;
        let mut ordered_count = 0_usize;
        while i < len1 || j < len2 {
            let n1 = match nums1.get(i) {
                None => i32::MAX,
                Some(&val) => val,
            };
            let n2 = match nums2.get(j) {
                None => i32::MAX,
                Some(&val) => val,
            };
            if n1 <= n2 {
                i += 1;
                res[switch_idx] = n1;
            } else {
                j += 1;
                res[switch_idx] = n2;
            }
            ordered_count += 1;

            if ordered_count == target_idx {
                switch_idx = 1;
            }
            if ordered_count == target_idx + 1 {
                break;
            }
        }

        if len_sum % 2 == 0 {
            res.iter().sum::<i32>() as f64 / 2.0
        } else {
            res[switch_idx] as f64
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
