//Given a signed 32-bit integer x, return x with its digits reversed. If
//reversing x causes the value to go outside the signed 32-bit integer range [-2³¹, 2³¹ -
// 1], then return 0.
//
// Assume the environment does not allow you to store 64-bit integers (signed
//or unsigned).
//
//
// Example 1:
//
//
//Input: x = 123
//Output: 321
//
//
// Example 2:
//
//
//Input: x = -123
//Output: -321
//
//
// Example 3:
//
//
//Input: x = 120
//Output: 21
//
//
//
// Constraints:
//
//
// -2³¹ <= x <= 2³¹ - 1
//
//
// Related Topics Math 👍 10847 👎 12284

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut res = 0_i64;
        let i32_max = i32::MAX as i64;
        let i32_min = i32::MIN as i64;

        while x != 0 {
            let rem = x % 10;

            let temp = res * 10 + rem as i64;
            if x > 0 && temp > i32_max {
                res = 0;
                break;
            } else if x < 0 && temp < i32_min {
                res = 0;
                break;
            } else {
                res = temp;
            }

            x /= 10;
        }

        res as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)
