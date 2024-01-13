//Given an array of intervals where intervals[i] = [starti, endi], merge all
//overlapping intervals, and return an array of the non-overlapping intervals that
//cover all the intervals in the input.
//
//
// Example 1:
//
//
//Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
//Output: [[1,6],[8,10],[15,18]]
//Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].
//
//
// Example 2:
//
//
//Input: intervals = [[1,4],[4,5]]
//Output: [[1,5]]
//Explanation: Intervals [1,4] and [4,5] are considered overlapping.
//
//
//
// Constraints:
//
//
// 1 <= intervals.length <= 10⁴
// intervals[i].length == 2
// 0 <= starti <= endi <= 10⁴
//
//
// Related Topics Array Sorting 👍 19945 👎 674

#![allow(dead_code)]
//   测试用例              期望结果
// [[1,4],[0,4]]         [[0,4]]
// [[1,4],[2,3]]         [[1,4]]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable();

        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut res = vec![];

        let mut iter = intervals.iter().peekable();
        while let Some(curr) = iter.next() {
            min = std::cmp::min(min, curr[0]);
            max = std::cmp::max(max, curr[1]);
            match iter.peek() {
                None => {
                    res.push(vec![min, max]);
                }
                Some(next) => {
                    if next[0] > max {
                        // 无交集
                        res.push(vec![min, max]);
                        min = i32::MAX;
                        max = i32::MIN;
                    }
                }
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
