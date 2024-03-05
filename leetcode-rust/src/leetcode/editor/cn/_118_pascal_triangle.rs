//Given an integer numRows, return the first numRows of Pascal's triangle.
//
// In Pascal's triangle, each number is the sum of the two numbers directly
//above it as shown:
//
//
// Example 1:
// Input: numRows = 5
//Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
//
// Example 2:
// Input: numRows = 1
//Output: [[1]]
//
//
// Constraints:
//
//
// 1 <= numRows <= 30
//
//
// Related Topics Array Dynamic Programming 👍 11022 👎 358

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        Self::solution1(num_rows)
        //Self::solution2(num_rows)
    }

    pub fn solution1(num_rows: i32) -> Vec<Vec<i32>> {
        (1..=num_rows)
            .map(|cap| {
                let mut row = Vec::with_capacity(cap as usize);

                (0..cap).for_each(|idx| {
                    if idx == 0 || idx == cap - 1 {
                        row.push(1);
                    } else {
                        row.push(row[idx as usize - 1] * (cap - idx) / idx);
                    }
                });

                row
            })
            .collect::<Vec<_>>()
    }

    pub fn solution2(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = (1..=num_rows)
            .map(|cap| vec![1; cap as usize])
            .collect::<Vec<_>>();
        for i in 0..res.len() {
            let row_len = res[i].len();
            for j in 0..row_len {
                if j != 0 && j != row_len - 1 {
                    res[i][j] = res[i - 1][j - 1] + res[i - 1][j];
                }
            }
        }
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
