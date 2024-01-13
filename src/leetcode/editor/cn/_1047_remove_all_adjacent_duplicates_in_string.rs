//You are given a string s consisting of lowercase English letters. A duplicate
//removal consists of choosing two adjacent and equal letters and removing them.
//
// We repeatedly make duplicate removals on s until we no longer can.
//
// Return the final string after all such duplicate removals have been made. It
//can be proven that the answer is unique.
//
//
// Example 1:
//
//
//Input: s = "abbaca"
//Output: "ca"
//Explanation:
//For example, in "abbaca" we could remove "bb" since the letters are adjacent
//and equal, and this is the only possible move.  The result of this move is that
//the string is "aaca", of which only "aa" is possible, so the final string is
//"ca".
//
//
// Example 2:
//
//
//Input: s = "azxxzy"
//Output: "ay"
//
//
//
// Constraints:
//
//
// 1 <= s.length <= 10⁵
// s consists of lowercase English letters.
//
//
// Related Topics String Stack 👍 6322 👎 237

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        //Self::brute_force(s)
        Self::use_stack(s)
    }

    /// Time Complexity: O(n^2)
    ///
    /// Space Complexity: O(1)
    fn brute_force(mut s: String) -> String {
        let bytes = unsafe { s.as_mut_vec() };
        let mut curr = 0;
        let mut next = 1;

        while next < bytes.len() {
            if bytes[curr] == bytes[next] {
                loop {
                    if curr == 0 || next == bytes.len() - 1 {
                        break;
                    }
                    if bytes[curr - 1] == bytes[next + 1] {
                        curr -= 1;
                        next += 1;
                    } else {
                        break;
                    }
                }
                bytes.drain(curr..=next);
                curr = 0;
                next = 1;
            } else {
                curr = next;
                next += 1;
            }
        }

        s
    }

    /// Time Complexity: O(n)
    ///
    /// Space Complexity: O(n)
    fn use_stack(s: String) -> String {
        let mut stack = String::with_capacity(s.len() / 2);

        for ch in s.chars() {
            match stack.pop() {
                None => {
                    stack.push(ch);
                }
                Some(top) => {
                    if top != ch {
                        stack.push(top);
                        stack.push(ch);
                    }
                }
            }
        }

        stack
    }
}
//leetcode submit region end(Prohibit modification and deletion)
