//Given a string s and an integer k, reverse the first k characters for every 2
//k characters counting from the start of the string.
//
// If there are fewer than k characters left, reverse all of them. If there are
//less than 2k but greater than or equal to k characters, then reverse the first
//k characters and leave the other as original.
//
//
// Example 1:
// Input: s = "abcdefg", k = 2
//Output: "bacdfeg"
//
// Example 2:
// Input: s = "abcd", k = 2
//Output: "bacd"
//
//
// Constraints:
//
//
// 1 <= s.length <= 10⁴
// s consists of only lowercase English letters.
// 1 <= k <= 10⁴
//
//
// Related Topics Two Pointers String 👍 1822 👎 3568

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        Self::two_pointers(s, k as usize)
    }

    fn two_pointers(mut s: String, k: usize) -> String {
        let len = s.len();
        let p = unsafe { s.as_bytes_mut() };
        let mut begin = 0_usize;
        let mut end = begin + k;

        loop {
            if begin >= len {
                break;
            }
            if end > len {
                end = len;
            }

            let mut l = begin;
            let mut r = end - 1;
            while l < r {
                p.swap(l, r);
                l += 1;
                r -= 1;
            }
            begin += 2 * k;
            end = begin + k;
        }

        s
    }
}
//leetcode submit region end(Prohibit modification and deletion)
