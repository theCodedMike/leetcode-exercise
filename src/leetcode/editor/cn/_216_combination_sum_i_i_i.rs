//Find all valid combinations of k numbers that sum up to n such that the
//following conditions are true:
//
//
// Only numbers 1 through 9 are used.
// Each number is used at most once.
//
//
// Return a list of all possible valid combinations. The list must not contain
//the same combination twice, and the combinations may be returned in any order.
//
//
// Example 1:
//
//
//Input: k = 3, n = 7
//Output: [[1,2,4]]
//Explanation:
//1 + 2 + 4 = 7
//There are no other valid combinations.
//
// Example 2:
//
//
//Input: k = 3, n = 9
//Output: [[1,2,6],[1,3,5],[2,3,4]]
//Explanation:
//1 + 2 + 6 = 9
//1 + 3 + 5 = 9
//2 + 3 + 4 = 9
//There are no other valid combinations.
//
//
// Example 3:
//
//
//Input: k = 4, n = 1
//Output: []
//Explanation: There are no valid combinations.
//Using 4 different numbers in the range [1,9], the smallest sum we can get is 1
//+2+3+4 = 10 and since 10 > 1, there are no valid combination.
//
//
//
// Constraints:
//
//
// 2 <= k <= 9
// 1 <= n <= 60
//
//
// Related Topics 数组 回溯 👍 800 👎 0

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        //Self::backtracking(k, n)

        Self::subset_enum(k, n)
    }

    fn backtracking(k: i32, n: i32) -> Vec<Vec<i32>> {
        const DFS: fn(i32, i32, i32, &mut Vec<i32>, &mut Vec<Vec<i32>>) =
            |idx, k, n, combine, res| {
                if k == 0 {
                    if n == 0 {
                        res.push(combine.clone());
                    }
                    return;
                }

                for i in idx..=9 {
                    if n < i || (n == i && k != 1) {
                        break;
                    }
                    combine.push(i);
                    DFS(i + 1, k - 1, n - i, combine, res);
                    combine.pop();
                }
            };
        let mut res = vec![];

        DFS(1, k, n, &mut vec![], &mut res);

        res
    }

    fn subset_enum(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut combine = vec![];

        let check = |mask, k, n, combine: &mut Vec<i32>| {
            combine.clear();

            let mut sum = 0;
            for i in 0..9 {
                if (1 << i) & mask != 0 {
                    sum += i + 1;
                    combine.push(i + 1);
                }
            }

            combine.len() as i32 == k && sum == n
        };

        for mask in 0..(1 << 9) {
            if check(mask, k, n, &mut combine) {
                res.push(combine.clone());
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
