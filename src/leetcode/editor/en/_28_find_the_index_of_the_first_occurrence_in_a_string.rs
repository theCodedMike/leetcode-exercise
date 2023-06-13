//Given two strings needle and haystack, return the index of the first
//occurrence of needle in haystack, or -1 if needle is not part of haystack.
//
//
// Example 1:
//
//
//Input: haystack = "sadbutsad", needle = "sad"
//Output: 0
//Explanation: "sad" occurs at index 0 and 6.
//The first occurrence is at index 0, so we return 0.
//
//
// Example 2:
//
//
//Input: haystack = "leetcode", needle = "leeto"
//Output: -1
//Explanation: "leeto" did not occur in "leetcode", so we return -1.
//
//
//
// Constraints:
//
//
// 1 <= haystack.length, needle.length <= 10⁴
// haystack and needle consist of only lowercase English characters.
//
//
// Related Topics Two Pointers String String Matching 👍 3669 👎 194

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        for (idx, val) in haystack.as_bytes().windows(needle.len()).enumerate() {
            if needle.as_bytes().eq(val) {
                return idx as i32;
            }
        }

        -1
    }
}
//leetcode submit region end(Prohibit modification and deletion)
