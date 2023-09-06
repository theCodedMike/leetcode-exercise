//You are given an array of strings tokens that represents an arithmetic
//expression in a Reverse Polish Notation.
//
// Evaluate the expression. Return an integer that represents the value of the
//expression.
//
// Note that:
//
//
// The valid operators are '+', '-', '*', and '/'.
// Each operand may be an integer or another expression.
// The division between two integers always truncates toward zero.
// There will not be any division by zero.
// The input represents a valid arithmetic expression in a reverse polish
//notation.
// The answer and all the intermediate calculations can be represented in a 32-
//bit integer.
//
//
//
// Example 1:
//
//
//Input: tokens = ["2","1","+","3","*"]
//Output: 9
//Explanation: ((2 + 1) * 3) = 9
//
//
// Example 2:
//
//
//Input: tokens = ["4","13","5","/","+"]
//Output: 6
//Explanation: (4 + (13 / 5)) = 6
//
//
// Example 3:
//
//
//Input: tokens = ["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
//Output: 22
//Explanation: ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
//= ((10 * (6 / (12 * -11))) + 17) + 5
//= ((10 * (6 / -132)) + 17) + 5
//= ((10 * 0) + 17) + 5
//= (0 + 17) + 5
//= 17 + 5
//= 22
//
//
//
// Constraints:
//
//
// 1 <= tokens.length <= 10⁴
// tokens[i] is either an operator: "+", "-", "*", or "/", or an integer in the
//range [-200, 200].
//
//
// Related Topics Array Math Stack 👍 6451 👎 936

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];

        for token in &tokens {
            let token = token.as_str();
            match token {
                "+" | "-" | "*" | "/" => {
                    let r_operand = stack.pop().unwrap_or_default();
                    let l_operand = stack.pop().unwrap_or_default();
                    stack.push(Self::calc(l_operand, r_operand, token));
                }
                _ => {
                    let val = token.parse::<i32>().unwrap_or_default();
                    stack.push(val);
                }
            }
        }

        stack.pop().unwrap_or_default()
    }

    fn calc(l_operand: i32, r_operand: i32, operator: &str) -> i32 {
        match operator {
            "+" => l_operand + r_operand,
            "-" => l_operand - r_operand,
            "*" => l_operand * r_operand,
            "/" => l_operand / r_operand,
            _ => panic!("unsupported operator"),
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
