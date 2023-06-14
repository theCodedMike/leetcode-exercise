//Given two binary strings a and b, return their sum as a binary string.
//
//
// Example 1:
// Input: a = "11", b = "1"
//Output: "100"
//
// Example 2:
// Input: a = "1010", b = "1011"
//Output: "10101"
//
//
// Constraints:
//
//
// 1 <= a.length, b.length <= 10⁴
// a and b consist only of '0' or '1' characters.
// Each string does not contain leading zeros except for the zero itself.
//
//
// Related Topics Math String Bit Manipulation Simulation 👍 8115 👎 801

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut result = "".to_string();
        let mut carry = 0;
        let mut a_stack = a.into_bytes();
        let mut b_stack = b.into_bytes();

        while !a_stack.is_empty() || !b_stack.is_empty() {
            let sum = get_digit(a_stack.as_mut()) + get_digit(b_stack.as_mut()) + carry;
            carry = sum / 2;
            result.insert_str(0, (sum % 2).to_string().as_str());
        }

        if carry == 1 {
            result.insert_str(0, "1");
        }

        result
    }
}

fn get_digit(stack: &mut Vec<u8>) -> usize {
    match stack.pop() {
        None => 0,
        Some(x) => match x {
            48 => 0,
            _ => 1,
        },
    }
}
//leetcode submit region end(Prohibit modification and deletion)
