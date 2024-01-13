//You are given an array of non-overlapping intervals intervals where intervals[
//i] = [starti, endi] represent the start and the end of the iᵗʰ interval and
//intervals is sorted in ascending order by starti. You are also given an interval
//newInterval = [start, end] that represents the start and end of another interval.
//
// Insert newInterval into intervals such that intervals is still sorted in
//ascending order by starti and intervals still does not have any overlapping
//intervals (merge overlapping intervals if necessary).
//
// Return intervals after the insertion.
//
//
// Example 1:
//
//
//Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
//Output: [[1,5],[6,9]]
//
//
// Example 2:
//
//
//Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
//Output: [[1,2],[3,10],[12,16]]
//Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].
//
//
//
// Constraints:
//
//
// 0 <= intervals.length <= 10⁴
// intervals[i].length == 2
// 0 <= starti <= endi <= 10⁵
// intervals is sorted by starti in ascending order.
// newInterval.length == 2
// 0 <= start <= end <= 10⁵
//
//
// Related Topics Array 👍 8676 👎 618

#![allow(dead_code)]

pub struct Solution;
//   intervals             newInterval           Output
// []                      [5,7]                 [[5,7]]
// [[1,5]]                 [6,8]                 [[1,5],[6,8]]
// [[1,5]]                 [0,3]                 [[0,5]]
// [[3,5],[12,15]]         [6,6]                 [[3,5],[6,6],[12,15]]        这个示例需要非常小心
// [[2,5],[6,7],[8,9]]     [0,1]                 [[0,1],[2,5],[6,7],[8,9]]

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let len = intervals.len();
        if len == 0 {
            res.push(new_interval);
            return res;
        }

        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut at_mid = false; //
        for (idx, curr) in intervals.iter().enumerate() {
            if curr[1] < new_interval[0] {
                res.push(curr.clone());
                if idx == len - 1 {
                    // new_interval 大于所有的 intervals
                    res.push(new_interval.clone());
                }
            } else if new_interval[1] < curr[0] {
                if idx == 0 || !at_mid {
                    // new_interval 小于所有的 intervals
                    res.push(new_interval.clone());

                    at_mid = true;
                }
                res.push(curr.clone());
            } else {
                at_mid = true;
                Solution::process_min_max(&mut min, curr[0], &mut max, curr[1], false, &mut res);
                // peek next
                match intervals.get(idx + 1) {
                    None => {
                        Solution::process_min_max(
                            &mut min,
                            new_interval[0],
                            &mut max,
                            new_interval[1],
                            true,
                            &mut res,
                        );
                    }
                    Some(next) => {
                        if new_interval[1] < next[0] {
                            Solution::process_min_max(
                                &mut min,
                                new_interval[0],
                                &mut max,
                                new_interval[1],
                                true,
                                &mut res,
                            );
                        }
                    }
                }
            }
        }

        res
    }

    fn process_min_max(
        min: &mut i32,
        min2: i32,
        max: &mut i32,
        max2: i32,
        process_next: bool,
        res: &mut Vec<Vec<i32>>,
    ) {
        *min = std::cmp::min(*min, min2);
        *max = std::cmp::max(*max, max2);
        if process_next {
            res.push(vec![*min, *max]);
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
