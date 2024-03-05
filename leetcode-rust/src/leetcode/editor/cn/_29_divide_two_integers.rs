//Given two integers dividend and divisor, divide two integers without using
//multiplication, division, and mod operator.
//
// The integer division should truncate toward zero, which means losing its
//fractional part. For example, 8.345 would be truncated to 8, and -2.7335 would be
//truncated to -2.
//
// Return the quotient after dividing dividend by divisor.
//
// Note: Assume we are dealing with an environment that could only store
//integers within the 32-bit signed integer range: [−2³¹, 2³¹ − 1]. For this problem, if
//the quotient is strictly greater than 2³¹ - 1, then return 2³¹ - 1, and if the
//quotient is strictly less than -2³¹, then return -2³¹.
//
//
// Example 1:
//
//
//Input: dividend = 10, divisor = 3
//Output: 3
//Explanation: 10/3 = 3.33333.. which is truncated to 3.
//
//
// Example 2:
//
//
//Input: dividend = 7, divisor = -3
//Output: -2
//Explanation: 7/-3 = -2.33333.. which is truncated to -2.
//
//
//
// Constraints:
//
//
// -2³¹ <= dividend, divisor <= 2³¹ - 1
// divisor != 0
//
//
// Related Topics Math Bit Manipulation 👍 4412 👎 13588

#![allow(dead_code)]

pub struct Solution;
// 	测试用例:-2147483648
// 			-1

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        // 考虑被除数为最小值的情况
        if dividend == i32::MIN {
            if divisor == 1 {
                return i32::MIN;
            }
            if divisor == -1 {
                return i32::MAX;
            }
        }

        // 考虑除数是最小值的情况
        if divisor == i32::MIN {
            if dividend == i32::MIN {
                return 1;
            }
            return 0;
        }

        // 考虑除数是0的情况
        if divisor == 0 {
            return 0;
        }

        // 一般情况，使用二分查找
        // 将所有的正数取相反数，这样就只需要考虑一种情况
        let mut rev = false;
        if dividend > 0 {
            dividend = -dividend;
            rev = !rev;
        }
        if divisor > 0 {
            divisor = -divisor;
            rev = !rev;
        }

        let mut ans = 0;
        let mut left = 1;
        let mut right = i32::MAX;

        while left <= right {
            // 注意溢出，且不能使用除法
            let mid = left + ((right - left) >> 1);
            if Solution::quick_add(dividend, divisor, mid) {
                ans = mid;
                if mid == i32::MAX {
                    // 注意溢出
                    break;
                }
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        if rev {
            -ans
        } else {
            ans
        }
    }

    /// 快速乘
    /// x、y是负数，z是正数
    /// 判断 z * y >= x 是否成立
    fn quick_add(x: i32, y: i32, mut z: i32) -> bool {
        let mut result = 0;
        let mut add = y;

        while z > 0 {
            if z & 1 > 0 {
                // 需要保证 result + add >= x
                if result < x - add {
                    return false;
                }
                result += add;
            }
            if z != 1 {
                // 需要保证 add + add >= x
                if add < x - add {
                    return false;
                }
                add += add;
            }

            z >>= 1;
        }

        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)
