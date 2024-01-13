//Given a string s consisting of words and spaces, return the length of the
//last word in the string.
//
// A word is a maximal substring consisting of non-space characters only.
//
//
// Example 1:
//
//
//Input: s = "Hello World"
//Output: 5
//Explanation: The last word is "World" with length 5.
//
//
// Example 2:
//
//
//Input: s = "   fly me   to   the moon  "
//Output: 4
//Explanation: The last word is "moon" with length 4.
//
//
// Example 3:
//
//
//Input: s = "luffy is still joyboy"
//Output: 6
//Explanation: The last word is "joyboy" with length 6.
//
//
//
// Constraints:
//
//
// 1 <= s.length <= 10⁴
// s consists of only English letters and spaces ' '.
// There will be at least one word in s.
//
//
// Related Topics String 👍 3317 👎 176

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut len = 0;

        for c in s.chars().rev() {
            if c != ' ' {
                len += 1;
            } else {
                if len > 0 {
                    break;
                }
            }
        }

        len
    }
}
//leetcode submit region end(Prohibit modification and deletion)
