//Given an m x n 2D binary grid grid which represents a map of '1's (land) and
//'0's (water), return the number of islands.
//
// An island is surrounded by water and is formed by connecting adjacent lands
//horizontally or vertically. You may assume all four edges of the grid are all
//surrounded by water.
//
//
// Example 1:
//
//
//Input: grid = [
//  ["1","1","1","1","0"],
//  ["1","1","0","1","0"],
//  ["1","1","0","0","0"],
//  ["0","0","0","0","0"]
//]
//Output: 1
//
//
// Example 2:
//
//
//Input: grid = [
//  ["1","1","0","0","0"],
//  ["1","1","0","0","0"],
//  ["0","0","1","0","0"],
//  ["0","0","0","1","1"]
//]
//Output: 3
//
//
//
// Constraints:
//
//
// m == grid.length
// n == grid[i].length
// 1 <= m, n <= 300
// grid[i][j] is '0' or '1'.
//
//
// Related Topics Array Depth-First Search Breadth-First Search Union Find
//Matrix 👍 21200 👎 456

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;

        let mut islands = 0;
        for i in 0..rows {
            for j in 0..cols {
                if grid[i as usize][j as usize] == '1' {
                    Self::dfs(&mut grid, i, j, rows, cols);
                    islands += 1;
                }
            }
        }

        islands
    }

    fn dfs(grid: &mut Vec<Vec<char>>, i: i32, j: i32, rows: i32, cols: i32) {
        // idx check
        if !(0 <= i && i < rows && 0 <= j && j < cols) {
            return;
        }

        // water and visited land should be ignored
        if grid[i as usize][j as usize] != '1' {
            return;
        }

        // mark some land to visited
        grid[i as usize][j as usize] = '2';

        Self::dfs(grid, i - 1, j, rows, cols); // top
        Self::dfs(grid, i + 1, j, rows, cols); // down
        Self::dfs(grid, i, j - 1, rows, cols); // left
        Self::dfs(grid, i, j + 1, rows, cols); // right
    }
}
//leetcode submit region end(Prohibit modification and deletion)
