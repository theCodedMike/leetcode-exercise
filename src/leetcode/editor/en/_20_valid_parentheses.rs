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
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut brackets = vec![];
        let mut is_valid = true;

        for char in s.chars() {
            match char {
                '(' | '{' | '[' => {
                    brackets.push(char);
                }
                _ => {
                    if brackets.is_empty() {
                        is_valid = false;
                        break;
                    }
                    let pop = brackets.pop().unwrap();
                    if !is_two_brackets_match(pop, char) {
                        is_valid = false;
                        break;
                    }
                }
            }
        }

        is_valid && brackets.is_empty()
    }
}

fn is_two_brackets_match(left: char, right: char) -> bool {
    match (left, right) {
        ('(', ')') | ('{', '}') | ('[', ']') => true,
        _ => false,
    }
}
//leetcode submit region end(Prohibit modification and deletion)
