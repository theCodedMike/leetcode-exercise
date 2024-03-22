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

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        //Self::brute_force(intervals)

        Self::sorting(intervals)
    }

    fn brute_force(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable();

        let (mut start, mut end) = (i32::MAX, i32::MIN);
        let (mut res, len) = (vec![], intervals.len());
        for i in 0..len {
            start = std::cmp::min(start, intervals[i][0]);
            end = std::cmp::max(end, intervals[i][1]);

            if (i < len - 1 && end < intervals[i + 1][0]) || (i == len - 1) {
                res.push(vec![start, end]);
                (start, end) = (i32::MAX, i32::MIN);
            }
        }

        res
    }

    fn sorting(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable();

        let mut res: Vec<Vec<i32>> = vec![];
        for interval in intervals {
            let (l, r) = (interval[0], interval[1]);
            let len = res.len();

            if len == 0 || res[len - 1][1] < l {
                res.push(vec![l, r]);
            } else {
                res[len - 1][1] = std::cmp::max(res[len - 1][1], r);
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
