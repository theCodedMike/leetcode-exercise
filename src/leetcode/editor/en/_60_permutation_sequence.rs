//The set [1, 2, 3, ..., n] contains a total of n! unique permutations.
//
// By listing and labeling all of the permutations in order, we get the
//following sequence for n = 3:
//
//
// "123"
// "132"
// "213"
// "231"
// "312"
// "321"
//
//
// Given n and k, return the kᵗʰ permutation sequence.
//
//
// Example 1:
// Input: n = 3, k = 3
//Output: "213"
//
// Example 2:
// Input: n = 4, k = 9
//Output: "2314"
//
// Example 3:
// Input: n = 3, k = 1
//Output: "123"
//
//
// Constraints:
//
//
// 1 <= n <= 9
// 1 <= k <= n!
//
//
// Related Topics Math Recursion 👍 5999 👎 448

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn get_permutation(n: i32, mut k: i32) -> String {
        let mut res = "".to_string();
        let mut ori = (1..=n).map(|i| i.to_string()).collect::<Vec<_>>();

        for i in (1..=n).rev() {
            let fact = (1..i).product::<i32>();
            for j in 1..=i {
                if j * fact >= k {
                    res.push_str(&ori.remove(j as usize - 1));
                    k -= (j - 1) * fact;
                    break;
                }
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
