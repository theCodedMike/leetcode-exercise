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
use std::ops::{Index, IndexMut};

impl Solution {
    pub fn reverse_words(s: String) -> String {
        //Self::use_std_split(s)
        //Self::use_custom_split(s)
        Self::use_stack(s)
    }

    fn use_std_split(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
    }

    fn use_custom_split(mut s: String) -> String {
        let reverse = |p: &mut [u8]| {
            let mut begin = 0;
            let mut end = p.len() - 1;

            while begin < end {
                p.swap(begin, end);
                begin += 1;
                end -= 1;
            }
        };

        let p = unsafe { s.as_bytes_mut() };
        reverse(p);

        let p = unsafe { s.as_mut_vec() };
        // Remove header spaces
        while p[0].is_ascii_whitespace() {
            p.remove(0);
        }
        // Remove trailing spaces
        while p[p.len() - 1].is_ascii_whitespace() {
            p.remove(p.len() - 1);
        }

        // Reverse the mid by word
        let mut space = 0;
        let mut slow = 0;
        let mut fast = 0;
        while fast < p.len() {
            if p[fast].is_ascii_whitespace() {
                space += 1;
                if space == 1 {
                    reverse(p.index_mut(slow..fast));
                    slow = fast + 1;
                    fast += 1;
                } else {
                    p.remove(fast);
                }
            } else {
                space = 0;
                fast += 1;
                if fast == p.len() {
                    reverse(p.index_mut(slow..fast));
                }
            }
        }

        s
    }

    fn use_stack(s: String) -> String {
        let len = s.len();
        let p = s.as_bytes();
        let mut stack = vec![];
        let mut begin = 0;
        let mut end = 0;
        for i in 0..len {
            if !p[i].is_ascii_whitespace() {
                if i == 0 || p[i - 1].is_ascii_whitespace() {
                    begin = i;
                }
                if i + 1 == len || p[i + 1].is_ascii_whitespace() {
                    end = i + 1;
                    stack.push((begin, end));
                }
            }
        }

        let mut res = String::with_capacity(len);
        let len = stack.len();
        for i in (0..len).rev() {
            res.push_str(s.index(stack[i].0..stack[i].1));
            if i != 0 {
                res.push(' ');
            }
        }
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
