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
        Self::brute_force(haystack, needle)
        //Self::kmp(haystack, needle)
    }

    /// Time Complexity: O(m * n)
    ///
    /// Space Complexity: O(1)
    fn brute_force(haystack: String, needle: String) -> i32 {
        let m = needle.len();
        let n = haystack.len();
        let needle = needle.as_bytes();
        let haystack = haystack.as_bytes();

        for end in m..=n {
            let mut all_equal = true;
            for i in end - m..end {
                if haystack[i] != needle[i - end + m] {
                    all_equal = false;
                    break;
                }
            }
            if all_equal {
                return (end - m) as i32;
            }
        }

        -1
    }

    /// Time Complexity: O(m + n)
    ///
    /// Space Complexity: O(m)
    fn kmp(haystack: String, needle: String) -> i32 {
        let get_next = |needle: &[u8]| -> Vec<usize> {
            let m = needle.len();
            let mut next = vec![0; m];
            let mut j = 0;
            for i in 1..m {
                while j > 0 && needle[i] != needle[j] {
                    j = next[j - 1];
                }
                if needle[i] == needle[j] {
                    j += 1;
                }
                next[i] = j;
            }
            next
        };

        let needle = needle.as_bytes();
        let next = get_next(needle);
        let m = needle.len();
        let n = haystack.len();
        let haystack = haystack.as_bytes();
        let mut j = 0;
        for i in 0..n {
            while j > 0 && haystack[i] != needle[j] {
                j = next[j - 1];
            }
            if haystack[i] == needle[j] {
                j += 1;
            }
            if j == m {
                return i as i32 - m as i32 + 1;
            }
        }

        -1
    }
}
//leetcode submit region end(Prohibit modification and deletion)
