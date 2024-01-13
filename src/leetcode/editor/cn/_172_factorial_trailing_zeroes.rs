//Given an integer n, return the number of trailing zeroes in n!.
//
// Note that n! = n * (n - 1) * (n - 2) * ... * 3 * 2 * 1.
//
//
// Example 1:
//
//
//Input: n = 3
//Output: 0
//Explanation: 3! = 6, no trailing zero.
//
//
// Example 2:
//
//
//Input: n = 5
//Output: 1
//Explanation: 5! = 120, one trailing zero.
//
//
// Example 3:
//
//
//Input: n = 0
//Output: 0
//
//
//
// Constraints:
//
//
// 0 <= n <= 10⁴
//
//
//
// Follow up: Could you write a solution that works in logarithmic time
//complexity?
//
// Related Topics Math 👍 2950 👎 1894

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn trailing_zeroes(mut n: i32) -> i32 {
        let mut res = 0;
        while n != 0 {
            n /= 5;
            res += n;
        }
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
