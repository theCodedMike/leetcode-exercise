//Given an array of points where points[i] = [xi, yi] represents a point on the
//X-Y plane, return the maximum number of points that lie on the same straight
//line.
//
//
// Example 1:
//
//
//Input: points = [[1,1],[2,2],[3,3]]
//Output: 3
//
//
// Example 2:
//
//
//Input: points = [[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]
//Output: 4
//
//
//
// Constraints:
//
//
// 1 <= points.length <= 300
// points[i].length == 2
// -10⁴ <= xi, yi <= 10⁴
// All the points are unique.
//
//
// Related Topics Array Hash Table Math Geometry 👍 3861 👎 434

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::{HashMap, HashSet};
use std::ops::Index;

#[derive(PartialEq, Eq, Hash)]
enum SlopeType {
    Positive,
    Negative,
}
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let len = points.len();
        if len == 1 {
            return 1;
        }
        let mut map = HashMap::new();

        for i in 0..len {
            for j in i + 1..len {
                let (x1, y1) = Self::get_x_y(&points, i);
                let (x2, y2) = Self::get_x_y(&points, j);

                let key = match x1 == x2 {
                    true => (SlopeType::Positive, i32::MAX, x1, 0),
                    false => match y1 == y2 {
                        true => (SlopeType::Positive, y1, i32::MAX, y1),
                        false => {
                            let x = x2 - x1;
                            let y = y2 - y1;
                            let abs_y = y.abs();
                            let abs_x = x.abs();
                            let b = y1 - (y / x * x1);
                            let gcd = Self::gcd(abs_x, abs_y);
                            if x * y > 0 {
                                // x 和 y 符号相同
                                (SlopeType::Positive, abs_y / gcd, abs_x / gcd, b)
                            } else {
                                (SlopeType::Negative, abs_y / gcd, abs_x / gcd, b)
                            }
                        }
                    },
                };

                let v = map.entry(key).or_insert(HashSet::new());
                v.insert((x1, y1));
                v.insert((x2, y2));
            }
        }

        let x3 = map.iter().filter(|(k, v)| v.len() >= 6).collect::<Vec<_>>();
        x3.len() as i32
    }

    fn get_x_y(points: &Vec<Vec<i32>>, idx: usize) -> (i32, i32) {
        let point = points.index(idx);
        (*point.index(0), *point.index(1))
    }

    fn gcd(mut m: i32, mut n: i32) -> i32 {
        if n == 0 {
            return m;
        }
        if m < n {
            return Self::gcd(n, m);
        }
        while n != 0 {
            let rem = m % n;
            m = n;
            n = rem;
        }
        m
    }
}
//leetcode submit region end(Prohibit modification and deletion)
