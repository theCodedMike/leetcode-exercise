//You are visiting a farm that has a single row of fruit trees arranged from
//left to right. The trees are represented by an integer array fruits where fruits[i]
// is the type of fruit the iᵗʰ tree produces.
//
// You want to collect as much fruit as possible. However, the owner has some
//strict rules that you must follow:
//
//
// You only have two baskets, and each basket can only hold a single type of
//fruit. There is no limit on the amount of fruit each basket can hold.
// Starting from any tree of your choice, you must pick exactly one fruit from
//every tree (including the start tree) while moving to the right. The picked
//fruits must fit in one of your baskets.
// Once you reach a tree with fruit that cannot fit in your baskets, you must
//stop.
//
//
// Given the integer array fruits, return the maximum number of fruits you can
//pick.
//
//
// Example 1:
//
//
//Input: fruits = [1,2,1]
//Output: 3
//Explanation: We can pick from all 3 trees.
//
//
// Example 2:
//
//
//Input: fruits = [0,1,2,2]
//Output: 3
//Explanation: We can pick from trees [1,2,2].
//If we had started at the first tree, we would only pick from trees [0,1].
//
//
// Example 3:
//
//
//Input: fruits = [1,2,3,2,2]
//Output: 4
//Explanation: We can pick from trees [2,3,2,2].
//If we had started at the first tree, we would only pick from trees [1,2].
//
//
//
// Constraints:
//
//
// 1 <= fruits.length <= 10⁵
// 0 <= fruits[i] < fruits.length
//
//
// Related Topics Array Hash Table Sliding Window 👍 4308 👎 302

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        //Self::brute_force(fruits)
        Self::sliding_window(fruits)
    }

    /// Time Limit Exceeded
    pub fn brute_force(fruits: Vec<i32>) -> i32 {
        let len = fruits.len();
        let mut set = HashSet::new();

        for width in (2..=len).rev() {
            let mut left = 0;
            let mut right = left + width;

            while right <= len {
                set.clear();
                for i in left..right {
                    set.insert(fruits[i]);
                    if set.len() > 2 {
                        break;
                    }
                }
                if set.len() == 2 {
                    return width as i32;
                }

                left += 1;
                right = left + width;
            }
        }

        fruits.len() as i32
    }

    pub fn sliding_window(fruits: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut left = 0;
        let mut res = 0;

        for right in 0..fruits.len() {
            map.entry(fruits[right])
                .and_modify(|v| *v += 1)
                .or_insert(1);
            while map.len() > 2 {
                if let Some(v) = map.get_mut(&fruits[left]) {
                    *v -= 1;
                }
                if map[&fruits[left]] == 0 {
                    map.remove(&fruits[left]);
                }
                left += 1;
            }

            res = std::cmp::max(res, right - left + 1);
        }

        res as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)
