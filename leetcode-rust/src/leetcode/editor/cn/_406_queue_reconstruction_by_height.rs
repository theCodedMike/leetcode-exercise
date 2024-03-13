//You are given an array of people, people, which are the attributes of some
//people in a queue (not necessarily in order). Each people[i] = [hi, ki] represents
//the iᵗʰ person of height hi with exactly ki other people in front who have a
//height greater than or equal to hi.
//
// Reconstruct and return the queue that is represented by the input array
//people. The returned queue should be formatted as an array queue, where queue[j] = [
//hj, kj] is the attributes of the jᵗʰ person in the queue (queue[0] is the person
//at the front of the queue).
//
//
// Example 1:
//
//
//Input: people = [[7,0],[4,4],[7,1],[5,0],[6,1],[5,2]]
//Output: [[5,0],[7,0],[5,2],[6,1],[4,4],[7,1]]
//Explanation:
//Person 0 has height 5 with no other people taller or the same height in front.
//
//Person 1 has height 7 with no other people taller or the same height in front.
//
//Person 2 has height 5 with two persons taller or the same height in front,
//which is person 0 and 1.
//Person 3 has height 6 with one person taller or the same height in front,
//which is person 1.
//Person 4 has height 4 with four people taller or the same height in front,
//which are people 0, 1, 2, and 3.
//Person 5 has height 7 with one person taller or the same height in front,
//which is person 1.
//Hence [[5,0],[7,0],[5,2],[6,1],[4,4],[7,1]] is the reconstructed queue.
//
//
// Example 2:
//
//
//Input: people = [[6,0],[5,0],[4,0],[3,2],[2,2],[1,4]]
//Output: [[4,0],[5,0],[2,2],[3,2],[1,4],[6,0]]
//
//
//
// Constraints:
//
//
// 1 <= people.length <= 2000
// 0 <= hi <= 10⁶
// 0 <= ki < people.length
// It is guaranteed that the queue can be reconstructed.
//
//
// Related Topics 树状数组 线段树 数组 排序 👍 1777 👎 0

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        //Self::consider_from_low_to_high(people)

        Self::consider_from_high_to_low(people)
    }

    fn consider_from_low_to_high(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));
        let len = people.len();
        let mut res = vec![vec![]; len];

        for p in people {
            let mut spaces = p[1] + 1;
            for i in 0..len {
                if res[i].is_empty() {
                    spaces -= 1;
                    if spaces == 0 {
                        res[i] = p;
                        break;
                    }
                }
            }
        }

        res
    }

    fn consider_from_high_to_low(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_unstable_by(|a, b| b[0].cmp(&a[0]).then(a[1].cmp(&b[1])));
        let mut res = Vec::with_capacity(people.len());

        for p in people {
            let idx = p[1] as usize;
            res.insert(idx, p);
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
