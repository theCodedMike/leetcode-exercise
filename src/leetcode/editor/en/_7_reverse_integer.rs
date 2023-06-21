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

use std::num::ParseIntError;

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut res;
        if x < 0 {
            if x == i32::MIN {
                return 0;
            }
            x = x.abs();
            res = "-".to_string();
        } else {
            res = "".to_string();
        }

        while x > 0 {
            let rem = x % 10;

            res += rem.to_string().as_str();

            x /= 10;
        }

        match res.parse::<i32>() {
            Ok(val) => val,
            Err(_) => 0,
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
