//Write a function that reverses a string. The input string is given as an
//array of characters s.
//
// You must do this by modifying the input array in-place with O(1) extra
//memory.
//
//
// Example 1:
// Input: s = ["h","e","l","l","o"]
//Output: ["o","l","l","e","h"]
//
// Example 2:
// Input: s = ["H","a","n","n","a","h"]
//Output: ["h","a","n","n","a","H"]
//
//
// Constraints:
//
//
// 1 <= s.length <= 10⁵
// s[i] is a printable ascii character.
//
//
// Related Topics Two Pointers String 👍 8026 👎 1136

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        //Self::two_pointers(s)
        //Self::two_pointers_2(s)
        Self::use_std(s)
    }

    /// 15ms 左右
    fn two_pointers(s: &mut Vec<char>) {
        let mut l = 0;
        let mut r = s.len() - 1;

        while l < r {
            s.swap(l, r);
            l += 1;
            r -= 1;
        }
    }

    /// 15ms 左右
    fn two_pointers_2(s: &mut Vec<char>) {
        let mut l = 0;
        let mut r = s.len() - 1;

        while l < r {
            let temp = s[l];
            s[l] = s[r];
            s[r] = temp;
            l += 1;
            r -= 1;
        }
    }

    /// 10ms 左右
    fn use_std(s: &mut Vec<char>) {
        s.reverse();
    }
}
//leetcode submit region end(Prohibit modification and deletion)
