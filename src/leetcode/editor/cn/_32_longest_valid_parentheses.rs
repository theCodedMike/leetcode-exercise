//Given a string containing just the characters '(' and ')', return the length
//of the longest valid (well-formed) parentheses substring.
//
//
// Example 1:
//
//
//Input: s = "(()"
//Output: 2
//Explanation: The longest valid parentheses substring is "()".
//
//
// Example 2:
//
//
//Input: s = ")()())"
//Output: 4
//Explanation: The longest valid parentheses substring is "()()".
//
//
// Example 3:
//
//
//Input: s = ""
//Output: 0
//
//
//
// Constraints:
//
//
// 0 <= s.length <= 3 * 10⁴
// s[i] is '(', or ')'.
//
//
// Related Topics String Dynamic Programming Stack 👍 11218 👎 356

#![allow(dead_code)]

pub struct Solution;
// 测试用例: "()(()"     "()(())"
// 期望结果: 2           6

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut valid_len = 0;
        let mut stack = vec![];
        let mut valid_len_coll = vec![0];
        // todo
        for c in s.chars() {
            match c {
                '(' => {
                    stack.push(c);
                }
                ')' => {
                    if !stack.is_empty() {
                        if Solution::is_match(stack.pop().unwrap(), c) {
                            valid_len += 1;
                        }
                    }
                }
                _ => {}
            }
        }
        if valid_len != 0 {
            valid_len_coll.push(valid_len * 2);
        }

        valid_len_coll.into_iter().max().unwrap()
    }

    fn is_match(left: char, right: char) -> bool {
        match (left, right) {
            ('(', ')') => true,
            _ => false,
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
