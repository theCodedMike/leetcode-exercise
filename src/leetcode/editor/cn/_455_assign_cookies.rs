//Assume you are an awesome parent and want to give your children some cookies.
//But, you should give each child at most one cookie.
//
// Each child i has a greed factor g[i], which is the minimum size of a cookie
//that the child will be content with; and each cookie j has a size s[j]. If s[j] >
//= g[i], we can assign the cookie j to the child i, and the child i will be
//content. Your goal is to maximize the number of your content children and output the
//maximum number.
//
//
// Example 1:
//
//
//Input: g = [1,2,3], s = [1,1]
//Output: 1
//Explanation: You have 3 children and 2 cookies. The greed factors of 3
//children are 1, 2, 3.
//And even though you have 2 cookies, since their size is both 1, you could
//only make the child whose greed factor is 1 content.
//You need to output 1.
//
//
// Example 2:
//
//
//Input: g = [1,2], s = [1,2,3]
//Output: 2
//Explanation: You have 2 children and 3 cookies. The greed factors of 2
//children are 1, 2.
//You have 3 cookies and their sizes are big enough to gratify all of the
//children,
//You need to output 2.
//
//
//
// Constraints:
//
//
// 1 <= g.length <= 3 * 10⁴
// 0 <= s.length <= 3 * 10⁴
// 1 <= g[i], s[j] <= 2³¹ - 1
//
//
// Related Topics 贪心 数组 双指针 排序 👍 830 👎 0

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::BTreeMap;
impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        //Self::brute_force(g, s)

        //Self::greedy(g, s)

        Self::solution_3(g, s)
    }

    fn brute_force(g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        s.sort_unstable();
        let mut map = g.into_iter().fold(BTreeMap::new(), |mut map, i| {
            map.entry(i).and_modify(|c| *c += 1).or_insert(1);
            map
        });

        let mut res = 0;
        for j in s {
            if map.is_empty() {
                break;
            }
            if let Some(mut entry) = map.first_entry() {
                if j >= *entry.key() {
                    res += 1;
                    *entry.get_mut() -= 1;
                    if *entry.get() == 0 {
                        entry.remove_entry();
                    }
                }
            }
        }

        res
    }

    fn greedy(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort_unstable();
        s.sort_unstable();

        let mut res = 0;
        let (m, n) = (g.len(), s.len());
        let (mut i, mut j) = (0, 0);
        while i < m && j < n {
            while j < n && g[i] > s[j] {
                j += 1;
            }
            if j < n {
                res += 1;
            }

            i += 1;
            j += 1;
        }

        res
    }

    fn solution_3(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort_unstable();
        s.sort_unstable();

        let mut j = 0;
        for i in s {
            if j < g.len() && g[j] <= i {
                j += 1;
            }
        }

        j as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)
