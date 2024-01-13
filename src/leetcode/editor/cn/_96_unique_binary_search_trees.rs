//Given an integer n, return the number of structurally unique BST's (binary
//search trees) which has exactly n nodes of unique values from 1 to n.
//
//
// Example 1:
//
//
//Input: n = 3
//Output: 5
//
//
// Example 2:
//
//
//Input: n = 1
//Output: 1
//
//
//
// Constraints:
//
//
// 1 <= n <= 19
//
//
// Related Topics Math Dynamic Programming Tree Binary Search Tree Binary Tree ?
//? 9711 👎 376

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let use_dp = false;

        if use_dp {
            Solution::dp_version(n)
        } else {
            Solution::catalan_version(n)
        }
    }

    fn dp_version(n: usize) -> i32 {
        let mut arr = vec![0; n + 1];
        arr[0] = 1;
        arr[1] = 1;

        for i in 2..=n {
            for j in 0..i {
                arr[i] += arr[j] * arr[i - j - 1];
            }
        }

        arr[n]
    }

    fn catalan_version(n: usize) -> i32 {
        let mut num = 1;

        for i in 1..=n {
            num = num * (4 * i - 2) / (i + 1);
        }

        num as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)
