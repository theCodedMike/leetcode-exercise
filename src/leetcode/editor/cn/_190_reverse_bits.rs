//Reverse bits of a given 32 bits unsigned integer.
//
// Note:
//
//
// Note that in some languages, such as Java, there is no unsigned integer type.
// In this case, both input and output will be given as a signed integer type.
//They should not affect your implementation, as the integer's internal binary
//representation is the same, whether it is signed or unsigned.
// In Java, the compiler represents the signed integers using 2's complement
//notation. Therefore, in Example 2 above, the input represents the signed integer -3
// and the output represents the signed integer -1073741825.
//
//
//
// Example 1:
//
//
//Input: n = 00000010100101000001111010011100
//Output:    964176192 (00111001011110000010100101000000)
//Explanation: The input binary string 00000010100101000001111010011100
//represents the unsigned integer 43261596, so return 964176192 which its binary
//representation is 00111001011110000010100101000000.
//
//
// Example 2:
//
//
//Input: n = 11111111111111111111111111111101
//Output:   3221225471 (10111111111111111111111111111111)
//Explanation: The input binary string 11111111111111111111111111111101
//represents the unsigned integer 4294967293, so return 3221225471 which its binary
//representation is 10111111111111111111111111111111.
//
//
//
// Constraints:
//
//
// The input must be a binary string of length 32
//
//
//
// Follow up: If this function is called many times, how would you optimize it?
//
//
// Related Topics Divide and Conquer Bit Manipulation 👍 4758 👎 1250

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
const M1: u32 = 0x55555555; // 01010101010101010101010101010101
const M2: u32 = 0x33333333; // 00110011001100110011001100110011
const M4: u32 = 0x0f0f0f0f; // 00001111000011110000111100001111
const M8: u32 = 0x00ff00ff; // 00000000111111110000000011111111

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        //Self::iter_helper(x)
        //Self::iter_use_bit_man(x)
        Self::divide_and_conquer(x)
    }

    /// 执行耗时:0 ms,击败了100.00% 的Rust用户
    /// 内存消耗:2.3 MB,击败了5.95% 的Rust用户
    pub fn iter_helper(x: u32) -> u32 {
        let s = format!("{:032b}", x);
        s.chars()
            .rev()
            .enumerate()
            .map(|(idx, c)| {
                if c == '0' {
                    0
                } else {
                    2_u32.pow((31 - idx) as u32)
                }
            })
            .sum()
    }

    /// 执行耗时:0 ms,击败了100.00% 的Rust用户
    /// 内存消耗:2 MB,击败了53.57% 的Rust用户
    pub fn iter_use_bit_man(mut x: u32) -> u32 {
        let mut res = 0;

        for i in 0..32 {
            res |= (x & 1) << (31 - i);
            x >>= 1;
            if x == 0 {
                break;
            }
        }

        res
    }

    /// 执行耗时:2 ms,击败了54.76% 的Rust用户
    /// 内存消耗:2 MB,击败了53.57% 的Rust用户
    pub fn divide_and_conquer(mut x: u32) -> u32 {
        x = x >> 1 & M1 | (x & M1) << 1;
        x = x >> 2 & M2 | (x & M2) << 2;
        x = x >> 4 & M4 | (x & M4) << 4;
        x = x >> 8 & M8 | (x & M8) << 8;
        x >> 16 | x << 16
    }
}
//leetcode submit region end(Prohibit modification and deletion)
