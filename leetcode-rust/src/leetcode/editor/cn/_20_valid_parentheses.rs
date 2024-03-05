//Given a string s containing just the characters '(', ')', '{', '}', '[' and ']
//', determine if the input string is valid.
//
// An input string is valid if:
//
//
// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
// Every close bracket has a corresponding open bracket of the same type.
//
//
//
// Example 1:
//
//
//Input: s = "()"
//Output: true
//
//
// Example 2:
//
//
//Input: s = "()[]{}"
//Output: true
//
//
// Example 3:
//
//
//Input: s = "(]"
//Output: false
//
//
//
// Constraints:
//
//
// 1 <= s.length <= 10⁴
// s consists of parentheses only '()[]{}'.
//
//
// Related Topics String Stack 👍 20192 👎 1231

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        //Self::use_stack(s)
        Self::optimize_use_stack(s)
    }

    /// Time Complexity: O(n)
    ///
    /// Space Complexity: O(n)
    fn use_stack(s: String) -> bool {
        let mut stack = vec![];
        let is_match = |l_char: char, r_char: char| -> bool {
            match (l_char, r_char) {
                ('(', ')') | ('{', '}') | ('[', ']') => true,
                _ => false,
            }
        };

        for ch in s.chars() {
            match ch {
                '(' | '{' | '[' => {
                    stack.push(ch);
                }
                ')' | '}' | ']' => match stack.pop() {
                    None => return false,
                    Some(l_ch) => {
                        if !is_match(l_ch, ch) {
                            return false;
                        }
                    }
                },
                _ => panic!("Unsupported char: {}", ch),
            }
        }

        stack.is_empty()
    }

    /// Time Complexity: O(n)
    ///
    /// Space Complexity: O(n + ∣Σ∣)
    fn optimize_use_stack(s: String) -> bool {
        let mut stack = vec![];
        let map = HashMap::from([(')', '('), ('}', '{'), (']', '[')]);

        for ch in s.chars() {
            match map.get(&ch) {
                None => {
                    stack.push(ch);
                }
                Some(&map_l_ch) => match stack.pop() {
                    None => return false,
                    Some(stack_l_ch) => {
                        if map_l_ch != stack_l_ch {
                            return false;
                        }
                    }
                },
            }
        }

        stack.is_empty()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
