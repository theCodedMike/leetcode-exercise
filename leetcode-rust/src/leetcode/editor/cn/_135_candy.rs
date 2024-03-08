//There are n children standing in a line. Each child is assigned a rating
//value given in the integer array ratings.
//
// You are giving candies to these children subjected to the following
//requirements:
//
//
// Each child must have at least one candy.
// Children with a higher rating get more candies than their neighbors.
//
//
// Return the minimum number of candies you need to have to distribute the
//candies to the children.
//
//
// Example 1:
//
//
//Input: ratings = [1,0,2]
//Output: 5
//Explanation: You can allocate to the first, second and third child with 2, 1,
//2 candies respectively.
//
//
// Example 2:
//
//
//Input: ratings = [1,2,2]
//Output: 4
//Explanation: You can allocate to the first, second and third child with 1, 2,
//1 candies respectively.
//The third child gets 1 candy because it satisfies the above two conditions.
//
//
//
// Constraints:
//
//
// n == ratings.length
// 1 <= n <= 2 * 10⁴
// 0 <= ratings[i] <= 2 * 10⁴
//
//
// Related Topics 贪心 数组 👍 1460 👎 0

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        //Self::double_traverse(ratings)

        Self::single_traverse(ratings)
    }

    fn double_traverse(ratings: Vec<i32>) -> i32 {
        let len = ratings.len();
        let mut left = vec![1; len];

        for i in 1..len {
            if ratings[i - 1] < ratings[i] {
                left[i] = left[i - 1] + 1;
            }
        }

        let (mut right, mut res) = (0, 0);
        for i in (0..len).rev() {
            if i != len - 1 && ratings[i] > ratings[i + 1] {
                right += 1;
            } else {
                right = 1;
            }
            res += std::cmp::max(right, left[i]);
        }

        res
    }

    fn single_traverse(ratings: Vec<i32>) -> i32 {
        let (mut res, len) = (1, ratings.len());
        let (mut inc, mut dec, mut pre) = (1, 0, 1);

        for i in 1..len {
            if ratings[i - 1] <= ratings[i] {
                dec = 0;
                pre = if ratings[i - 1] == ratings[i] {
                    1
                } else {
                    pre + 1
                };
                res += pre;
                inc = pre;
            } else {
                dec += 1;
                if dec == inc {
                    dec += 1;
                }
                res += dec;
                pre = 1;
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
