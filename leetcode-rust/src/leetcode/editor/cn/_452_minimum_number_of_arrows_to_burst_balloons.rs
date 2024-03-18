//There are some spherical balloons taped onto a flat wall that represents the
//XY-plane. The balloons are represented as a 2D integer array points where points[
//i] = [xstart, xend] denotes a balloon whose horizontal diameter stretches
//between xstart and xend. You do not know the exact y-coordinates of the balloons.
//
// Arrows can be shot up directly vertically (in the positive y-direction) from
//different points along the x-axis. A balloon with xstart and xend is burst by
//an arrow shot at x if xstart <= x <= xend. There is no limit to the number of
//arrows that can be shot. A shot arrow keeps traveling up infinitely, bursting any
//balloons in its path.
//
// Given the array points, return the minimum number of arrows that must be
//shot to burst all balloons.
//
//
// Example 1:
//
//
//Input: points = [[10,16],[2,8],[1,6],[7,12]]
//Output: 2
//Explanation: The balloons can be burst by 2 arrows:
//- Shoot an arrow at x = 6, bursting the balloons [2,8] and [1,6].
//- Shoot an arrow at x = 11, bursting the balloons [10,16] and [7,12].
//
//
// Example 2:
//
//
//Input: points = [[1,2],[3,4],[5,6],[7,8]]
//Output: 4
//Explanation: One arrow needs to be shot for each balloon for a total of 4
//arrows.
//
//
// Example 3:
//
//
//Input: points = [[1,2],[2,3],[3,4],[4,5]]
//Output: 2
//Explanation: The balloons can be burst by 2 arrows:
//- Shoot an arrow at x = 2, bursting the balloons [1,2] and [2,3].
//- Shoot an arrow at x = 4, bursting the balloons [3,4] and [4,5].
//
//
//
// Constraints:
//
//
// 1 <= points.length <= 10⁵
// points[i].length == 2
// -2³¹ <= xstart < xend <= 2³¹ - 1
//
//
// Related Topics 贪心 数组 排序 👍 943 👎 0

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        //Self::brute_force(points)

        Self::greedy(points)
    }

    /// Time Complexity: O(n^2)
    /// Space Complexity: O(n)
    fn brute_force(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
        let len = points.len();
        let mut burst = vec![false; len];
        let (mut idx, mut res) = (0, 0);

        let has_false = |idx: &mut usize, burst: &[bool]| {
            for i in 0..len {
                if !burst[i] {
                    *idx = i;
                    return true;
                }
            }
            false
        };

        while has_false(&mut idx, &burst) {
            res += 1;
            for j in idx..len {
                if points[j][0] <= points[idx][1] {
                    burst[j] = true;
                }
            }
        }

        res
    }

    /// Time Complexity: O(n*log(n))
    /// Space Complexity: O(n)
    fn greedy(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
        let (mut pos, mut res) = (points[0][1], 1);

        for p in points {
            if p[0] > pos {
                res += 1;
                pos = p[1];
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
