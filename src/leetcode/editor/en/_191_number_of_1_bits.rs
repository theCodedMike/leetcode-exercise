//Write a function that takes the binary representation of an unsigned integer
//and returns the number of '1' bits it has (also known as the Hamming weight).
//
// Note:
//
//
// Note that in some languages, such as Java, there is no unsigned integer type.
// In this case, the input will be given as a signed integer type. It should not
//affect your implementation, as the integer's internal binary representation is
//the same, whether it is signed or unsigned.
// In Java, the compiler represents the signed integers using 2's complement
//notation. Therefore, in Example 3, the input represents the signed integer. -3.
//
//
//
// Example 1:
//
//
//Input: n = 00000000000000000000000000001011
//Output: 3
//Explanation: The input binary string 00000000000000000000000000001011 has a
//total of three '1' bits.
//
//
// Example 2:
//
//
//Input: n = 00000000000000000000000010000000
//Output: 1
//Explanation: The input binary string 00000000000000000000000010000000 has a
//total of one '1' bit.
//
//
// Example 3:
//
//
//Input: n = 11111111111111111111111111111101
//Output: 31
//Explanation: The input binary string 11111111111111111111111111111101 has a
//total of thirty one '1' bits.
//
//
//
// Constraints:
//
//
// The input must be a binary string of length 32.
//
//
//
//Follow up: If this function is called many times, how would you optimize it?
//
// Related Topics Divide and Conquer Bit Manipulation 👍 5825 👎 1212

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::ops::Index;

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        //Self::iter_helper(n)
        Self::divide_and_conquer(n)
    }

    /// 执行耗时:0 ms,击败了100.00% 的Rust用户
    /// 内存消耗:2 MB,击败了90.87% 的Rust用户
    pub fn iter_helper(mut n: u32) -> i32 {
        let mut res = 0;

        while n != 0 {
            if (n & 1) == 1 {
                res += 1;
            }
            n >>= 1;
        }

        res
    }

    /// 执行耗时:0 ms,击败了100.00% 的Rust用户
    /// 内存消耗:2.1 MB,击败了47.49% 的Rust用户
    pub fn divide_and_conquer(n: u32) -> i32 {
        let bits = format!("{:32b}", n);
        Self::recursion_helper(&bits, 0, 32) as i32
    }

    fn recursion_helper(bits: &str, begin: usize, end: usize) -> usize {
        if begin + 1 == end {
            return if bits.index(begin..end) == "1" { 1 } else { 0 };
        }
        let mid = (begin + end) / 2;
        Self::recursion_helper(bits, begin, mid) + Self::recursion_helper(bits, mid, end)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
