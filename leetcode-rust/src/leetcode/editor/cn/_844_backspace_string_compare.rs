//Given two strings s and t, return true if they are equal when both are typed
//into empty text editors. '#' means a backspace character.
//
// Note that after backspacing an empty text, the text will continue empty.
//
//
// Example 1:
//
//
//Input: s = "ab#c", t = "ad#c"
//Output: true
//Explanation: Both s and t become "ac".
//
//
// Example 2:
//
//
//Input: s = "ab##", t = "c#d#"
//Output: true
//Explanation: Both s and t become "".
//
//
// Example 3:
//
//
//Input: s = "a#c", t = "b"
//Output: false
//Explanation: s becomes "c" while t becomes "b".
//
//
//
// Constraints:
//
//
// 1 <= s.length, t.length <= 200
// s and t only contain lowercase letters and '#' characters.
//
//
//
// Follow up: Can you solve it in O(n) time and O(1) space?
//
// Related Topics Two Pointers String Stack Simulation 👍 6705 👎 306

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::ops::Index;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        //Self::build_string(s, t)
        Self::two_pointers(s, t)
    }

    ///  Time Complexity: O(M+N)
    ///
    /// Space Complexity: O(M+N)
    pub fn build_string(s: String, t: String) -> bool {
        let build = |str: String| -> String {
            let mut res = String::new();

            for c in str.chars() {
                match c {
                    '#' => {
                        res.pop();
                    }
                    _ => {
                        res.push(c);
                    }
                }
            }

            res
        };

        build(s) == build(t)
    }

    ///  Time Complexity: O(M+N)
    ///
    /// Space Complexity: O(1)
    pub fn two_pointers(s: String, t: String) -> bool {
        let (mut s_idx, mut t_idx) = (s.len() as i32 - 1, t.len() as i32 - 1);
        let (mut s_sharp_count, mut t_sharp_count) = (0, 0);

        while s_idx >= 0 || t_idx >= 0 {
            while s_idx >= 0 {
                if s.chars().nth(s_idx as usize) == Some('#') {
                    s_sharp_count += 1;
                    s_idx -= 1;
                } else if s_sharp_count > 0 {
                    s_sharp_count -= 1;
                    s_idx -= 1;
                } else {
                    break;
                }
            }

            while t_idx >= 0 {
                if t.chars().nth(t_idx as usize) == Some('#') {
                    t_sharp_count += 1;
                    t_idx -= 1;
                } else if t_sharp_count > 0 {
                    t_sharp_count -= 1;
                    t_idx -= 1;
                } else {
                    break;
                }
            }

            // If two characters are different
            if s_idx >= 0
                && t_idx >= 0
                && s.chars().nth(s_idx as usize) != t.chars().nth(t_idx as usize)
            {
                return false;
            }
            // If char vs nothing
            if (s_idx >= 0) != (t_idx >= 0) {
                return false;
            }

            s_idx -= 1;
            t_idx -= 1;
        }

        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)
