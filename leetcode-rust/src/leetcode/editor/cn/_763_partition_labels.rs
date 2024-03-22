//You are given a string s. We want to partition the string into as many parts
//as possible so that each letter appears in at most one part.
//
// Note that the partition is done so that after concatenating all the parts in
//order, the resultant string should be s.
//
// Return a list of integers representing the size of these parts.
//
//
// Example 1:
//
//
//Input: s = "ababcbacadefegdehijhklij"
//Output: [9,7,8]
//Explanation:
//The partition is "ababcbaca", "defegde", "hijhklij".
//This is a partition so that each letter appears in at most one part.
//A partition like "ababcbacadefegde", "hijhklij" is incorrect, because it
//splits s into less parts.
//
//
// Example 2:
//
//
//Input: s = "eccbbbbdec"
//Output: [10]
//
//
//
// Constraints:
//
//
// 1 <= s.length <= 500
// s consists of lowercase English letters.
//
//
// Related Topics 贪心 哈希表 双指针 字符串 👍 1111 👎 0

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        Self::greedy(s)
    }

    fn greedy(s: String) -> Vec<i32> {
        let mut helper = [0; 26];
        let s_bytes = s.as_bytes();
        let len = s.len();

        for i in 0..len {
            let idx = (s_bytes[i] - b'a') as usize;
            helper[idx] = i;
        }

        let mut res = vec![];
        let (mut first, mut last) = (0, 0);
        for i in 0..len {
            let idx = (s_bytes[i] - b'a') as usize;
            last = std::cmp::max(last, helper[idx]);
            if i == last {
                res.push((last - first + 1) as i32);
                first = last + 1;
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
