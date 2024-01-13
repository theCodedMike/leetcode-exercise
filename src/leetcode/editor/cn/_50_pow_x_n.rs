//Implement pow(x, n), which calculates x raised to the power n (i.e., xⁿ).
//
//
// Example 1:
//
//
//Input: x = 2.00000, n = 10
//Output: 1024.00000
//
//
// Example 2:
//
//
//Input: x = 2.10000, n = 3
//Output: 9.26100
//
//
// Example 3:
//
//
//Input: x = 2.00000, n = -2
//Output: 0.25000
//Explanation: 2⁻² = 1/2² = 1/4 = 0.25
//
//
//
// Constraints:
//
//
// -100.0 < x < 100.0
// -2³¹ <= n <= 2³¹-1
// n is an integer.
// Either x is not zero or n > 0.
// -10⁴ <= xⁿ <= 10⁴
//
//
// Related Topics Math Recursion 👍 7693 👎 7825

#![allow(dead_code)]

pub struct Solution;
// 测试用例: x=0.00001         n= 2147483647        期望结果:0
// 测试用例: x=1.00000         n= 2147483647        期望结果:1
// 测试用例: x=2.00000         n=-2147483647        期望结果:0
// 测试用例: x=2.00000         n=-2147483648        期望结果:0
// 测试用例: x=1.0000000000001 n=-2147483648        期望结果:0.99979

//leetcode submit region begin(Prohibit modification and deletion)

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        } else if n & 1 == 0 {
            // n为偶数
            return Solution::my_pow(x * x, n / 2);
        } else {
            // n为奇数
            return if n > 0 { x } else { 1.0 / x } * Solution::my_pow(x * x, n / 2);
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
