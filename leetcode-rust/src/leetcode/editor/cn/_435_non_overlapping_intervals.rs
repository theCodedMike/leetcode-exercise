//Given an array of intervals intervals where intervals[i] = [starti, endi],
//return the minimum number of intervals you need to remove to make the rest of the
//intervals non-overlapping.
//
//
// Example 1:
//
//
//Input: intervals = [[1,2],[2,3],[3,4],[1,3]]
//Output: 1
//Explanation: [1,3] can be removed and the rest of the intervals are non-
//overlapping.
//
//
// Example 2:
//
//
//Input: intervals = [[1,2],[1,2],[1,2]]
//Output: 2
//Explanation: You need to remove two [1,2] to make the rest of the intervals
//non-overlapping.
//
//
// Example 3:
//
//
//Input: intervals = [[1,2],[2,3]]
//Output: 0
//Explanation: You don't need to remove any of the intervals since they're
//already non-overlapping.
//
//
//
// Constraints:
//
//
// 1 <= intervals.length <= 10⁵
// intervals[i].length == 2
// -5 * 10⁴ <= starti < endi <= 5 * 10⁴
//
//
// Related Topics 贪心 数组 动态规划 排序 👍 1118 👎 0

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        //Self::dp(intervals)

        Self::greedy(intervals)
    }

    /// Time Complexity: O(n^2)
    /// Space Complexity: O(n)
    fn dp(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let len = intervals.len();
        let mut f = vec![1; len];
        let mut max = 1;

        for i in 1..len {
            for j in 0..i {
                if intervals[i][0] >= intervals[j][1] {
                    f[i] = std::cmp::max(f[i], f[j] + 1);
                    if f[i] > max {
                        max = f[i];
                    }
                }
            }
        }

        (len - max) as i32
    }

    /// Time Complexity: O(n*log(n))
    /// Space Complexity: O(log(n))
    fn greedy(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
        let mut end = intervals[0][1];
        let mut res = 0;

        for i in 1..intervals.len() {
            if end <= intervals[i][0] {
                end = intervals[i][1];
            } else {
                res += 1;
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
