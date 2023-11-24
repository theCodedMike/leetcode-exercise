//Given a string s, check if it can be constructed by taking a substring of it
//and appending multiple copies of the substring together.
//
//
// Example 1:
//
//
//Input: s = "abab"
//Output: true
//Explanation: It is the substring "ab" twice.
//
//
// Example 2:
//
//
//Input: s = "aba"
//Output: false
//
//
// Example 3:
//
//
//Input: s = "abcabcabcabc"
//Output: true
//Explanation: It is the substring "abc" four times or the substring "abcabc"
//twice.
//
//
//
// Constraints:
//
//
// 1 <= s.length <= 10⁴
// s consists of lowercase English letters.
//
//
// Related Topics String String Matching 👍 6196 👎 494

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        //Self::brute_force(s)
        Self::kmp(s)
    }

    /// Time Complexity: O(n^2)
    ///
    /// Space Complexity: O(1)
    fn brute_force(s: String) -> bool {
        let s = s.as_bytes();
        let len = s.len();

        for sub_len in 1..=len / 2 {
            if len % sub_len == 0 {
                let mut all_equal = true;
                for i in sub_len..len {
                    if s[i] != s[i % sub_len] {
                        all_equal = false;
                        break;
                    }
                }
                if all_equal {
                    return true;
                }
            }
        }

        false
    }

    /// Time Complexity: O(n)
    ///
    /// Space Complexity: O(n)
    fn kmp(s: String) -> bool {
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

        let needle = s.as_bytes();
        let next = get_next(needle);
        let len = needle.len();
        next[len - 1] != 0 && len % (len - next[len - 1]) == 0
    }
}
//leetcode submit region end(Prohibit modification and deletion)
