//You are given two integer arrays nums1 and nums2, sorted in non-decreasing
//order, and two integers m and n, representing the number of elements in nums1 and
//nums2 respectively.
//
// Merge nums1 and nums2 into a single array sorted in non-decreasing order.
//
// The final sorted array should not be returned by the function, but instead
//be stored inside the array nums1. To accommodate this, nums1 has a length of m +
//n, where the first m elements denote the elements that should be merged, and the
//last n elements are set to 0 and should be ignored. nums2 has a length of n.
//
//
// Example 1:
//
//
//Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
//Output: [1,2,2,3,5,6]
//Explanation: The arrays we are merging are [1,2,3] and [2,5,6].
//The result of the merge is [1,2,2,3,5,6] with the underlined elements coming
//from nums1.
//
//
// Example 2:
//
//
//Input: nums1 = [1], m = 1, nums2 = [], n = 0
//Output: [1]
//Explanation: The arrays we are merging are [1] and [].
//The result of the merge is [1].
//
//
// Example 3:
//
//
//Input: nums1 = [0], m = 0, nums2 = [1], n = 1
//Output: [1]
//Explanation: The arrays we are merging are [] and [1].
//The result of the merge is [1].
//Note that because m = 0, there are no elements in nums1. The 0 is only there
//to ensure the merge result can fit in nums1.
//
//
//
// Constraints:
//
//
// nums1.length == m + n
// nums2.length == n
// 0 <= m, n <= 200
// 1 <= m + n <= 200
// -10⁹ <= nums1[i], nums2[j] <= 10⁹
//
//
//
// Follow up: Can you come up with an algorithm that runs in O(m + n) time?
//
// Related Topics Array Two Pointers Sorting 👍 10881 👎 1078

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn merge<'a>(nums1: &'a mut Vec<i32>, m: i32, nums2: &'a mut Vec<i32>, n: i32) {
        match (m, n) {
            (_, 0) => {}
            (m, n) => {
                let m = m as usize;
                let n = n as usize;
                let mut i = 0;
                let mut j = 0;
                let cap1 = m + n; // nums1的总长度
                let mut len1 = m; //num1中真实的元素个数，由m增长到m+n
                while i < cap1 && j < n {
                    let val1 = nums1[i];
                    let val2 = nums2[j];

                    if val1 <= val2 && i < len1 {
                        i += 1;
                    } else {
                        len1 += 1;
                        for idx in (i + 1..len1).rev() {
                            nums1[idx] = nums1[idx - 1];
                        }

                        nums1[i] = val2;

                        j += 1;
                        i += 1;
                    }
                }
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
