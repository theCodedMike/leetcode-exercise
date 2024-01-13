//Given two non-negative integers num1 and num2 represented as strings, return
//the product of num1 and num2, also represented as a string.
//
// Note: You must not use any built-in BigInteger library or convert the inputs
//to integer directly.
//
//
// Example 1:
// Input: num1 = "2", num2 = "3"
//Output: "6"
//
// Example 2:
// Input: num1 = "123", num2 = "456"
//Output: "56088"
//
//
// Constraints:
//
//
// 1 <= num1.length, num2.length <= 200
// num1 and num2 consist of digits only.
// Both num1 and num2 do not contain any leading zero, except the number 0
//itself.
//
//
// Related Topics Math String Simulation 👍 6359 👎 2903

#![allow(dead_code)]

pub struct Solution;
// u128::MAX: 340282366920938463463374607431768211455
//  u64::MAX: 18446744073709551615
//  u32::MAX: 4294967295

// 测试用例: "498828660196"                    "840477629533"
// 期望结果: "419254329864656431168468"
// 测试用例: "401716832807512840963"           "167141802233061013023557397451289113296441069"
// 期望结果: "67143675422804947379429215144664313370120390398055713625298709447"

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn multiply(_num1: String, _num2: String) -> String {
        let res = "".to_string();
        // todo!
        res
    }

    fn convert_char_2_digit(ch: char) -> u8 {
        match ch {
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _ => 0,
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
