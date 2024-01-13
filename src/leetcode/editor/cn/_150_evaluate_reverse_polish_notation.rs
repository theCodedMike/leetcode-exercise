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
const CALC: fn(i32, i32, &str) -> i32 = |left, right, operator| match operator {
    "+" => left + right,
    "-" => left - right,
    "*" => left * right,
    "/" => left / right,
    _ => panic!("Unsupported operator: {}", operator),
};

const IS_OPERATOR: fn(&str) -> bool = |token| match token {
    "+" | "-" | "*" | "/" => true,
    _ => false,
};

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        Self::simulate_stack(tokens)
        //Self::use_stack(tokens)
    }

    /// Time Complexity: O(n^2)
    ///
    /// Space Complexity: O(1)
    fn simulate_stack(mut tokens: Vec<String>) -> i32 {
        let len = tokens.len();

        for i in 0..len {
            if IS_OPERATOR(&tokens[i]) {
                let mut r = i - 1;
                while tokens[r].is_empty() {
                    r -= 1;
                }
                let right = tokens[r].parse::<i32>().unwrap();
                let mut l = r - 1;
                while tokens[l].is_empty() {
                    l -= 1;
                }
                let left = tokens[l].parse::<i32>().unwrap();
                tokens[i] = CALC(left, right, &tokens[i]).to_string();
                tokens[r].clear();
                tokens[l].clear();
            }
        }

        tokens[len - 1].parse::<i32>().unwrap()
    }

    /// Time Complexity: O(n)
    ///
    /// Space Complexity: O(n)
    fn use_stack(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::with_capacity(tokens.len() / 2);

        for ref token in tokens {
            if IS_OPERATOR(token) {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(CALC(left, right, token));
            } else {
                stack.push(token.parse::<i32>().unwrap())
            }
        }

        stack[0]
    }
}
//leetcode submit region end(Prohibit modification and deletion)
