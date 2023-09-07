//Given an input string s, reverse the order of the words.
//
// A word is defined as a sequence of non-space characters. The words in s will
//be separated by at least one space.
//
// Return a string of the words in reverse order concatenated by a single space.
//
//
// Note that s may contain leading or trailing spaces or multiple spaces
//between two words. The returned string should only have a single space separating the
//words. Do not include any extra spaces.
//
//
// Example 1:
//
//
//Input: s = "the sky is blue"
//Output: "blue is sky the"
//
//
// Example 2:
//
//
//Input: s = "  hello world  "
//Output: "world hello"
//Explanation: Your reversed string should not contain leading or trailing
//spaces.
//
//
// Example 3:
//
//
//Input: s = "a good   example"
//Output: "example good a"
//Explanation: You need to reduce multiple spaces between two words to a single
//space in the reversed string.
//
//
//
// Constraints:
//
//
// 1 <= s.length <= 10⁴
// s contains English letters (upper-case and lower-case), digits, and spaces '
//'.
// There is at least one word in s.
//
//
//
// Follow-up: If the string data type is mutable in your language, can you
//solve it in-place with O(1) extra space?
//
// Related Topics Two Pointers String 👍 7224 👎 4864

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::ops::Index;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        Self::std_split(s)
        //Self::custom_split(s)
    }

    fn std_split(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
    }

    fn custom_split(s: String) -> String {
        let s = s.as_str();
        let len = s.len();
        let mut first_p = 0;
        let mut second_p = 0;
        let mut words = vec![];
        while second_p < len {
            while first_p < len && s.index(first_p..first_p + 1) == " " {
                first_p += 1;
            }
            second_p = first_p;
            while second_p < len && s.index(second_p..second_p + 1) != " " {
                second_p += 1;
            }
            if first_p == second_p {
                break;
            }
            words.push(s.index(first_p..second_p));
            first_p = second_p;
        }

        words.into_iter().rev().collect::<Vec<_>>().join(" ")
    }
}
//leetcode submit region end(Prohibit modification and deletion)
