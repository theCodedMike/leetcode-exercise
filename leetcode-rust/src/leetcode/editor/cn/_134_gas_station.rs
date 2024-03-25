//There are n gas stations along a circular route, where the amount of gas at
//the iᵗʰ station is gas[i].
//
// You have a car with an unlimited gas tank and it costs cost[i] of gas to
//travel from the iᵗʰ station to its next (i + 1)ᵗʰ station. You begin the journey
//with an empty tank at one of the gas stations.
//
// Given two integer arrays gas and cost, return the starting gas station's
//index if you can travel around the circuit once in the clockwise direction,
//otherwise return -1. If there exists a solution, it is guaranteed to be unique
//
//
// Example 1:
//
//
//Input: gas = [1,2,3,4,5], cost = [3,4,5,1,2]
//Output: 3
//Explanation:
//Start at station 3 (index 3) and fill up with 4 unit of gas. Your tank = 0 + 4
// = 4
//Travel to station 4. Your tank = 4 - 1 + 5 = 8
//Travel to station 0. Your tank = 8 - 2 + 1 = 7
//Travel to station 1. Your tank = 7 - 3 + 2 = 6
//Travel to station 2. Your tank = 6 - 4 + 3 = 5
//Travel to station 3. The cost is 5. Your gas is just enough to travel back to
//station 3.
//Therefore, return 3 as the starting index.
//
//
// Example 2:
//
//
//Input: gas = [2,3,4], cost = [3,4,3]
//Output: -1
//Explanation:
//You can't start at station 0 or 1, as there is not enough gas to travel to
//the next station.
//Let's start at station 2 and fill up with 4 unit of gas. Your tank = 0 + 4 = 4
//
//Travel to station 0. Your tank = 4 - 3 + 2 = 3
//Travel to station 1. Your tank = 3 - 3 + 3 = 3
//You cannot travel back to station 2, as it requires 4 unit of gas but you
//only have 3.
//Therefore, you can't travel around the circuit once no matter where you start.
//
//
//
//
// Constraints:
//
//
// n == gas.length == cost.length
// 1 <= n <= 10⁵
// 0 <= gas[i], cost[i] <= 10⁴
//
//
// Related Topics 贪心 数组 👍 1568 👎 0

#![allow(dead_code)]
#![allow(unused_assignments)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        Self::brute_force(gas, cost)

        //Self::traverse_once(gas, cost)
    }

    fn brute_force(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let (len, mut start, mut curr_gas) = (gas.len(), 0, 0);

        'outer: for i in 0..len {
            start = len;
            for j in i..len {
                if gas[j] >= cost[j] {
                    start = j;
                    break;
                }
            }

            if start == len {
                break;
            }

            curr_gas = gas[start] - cost[start];
            for i in (start + 1..len).chain(0..start) {
                curr_gas += gas[i];
                if curr_gas < cost[i] {
                    continue 'outer;
                }
                curr_gas -= cost[i];
            }

            return start as i32;
        }

        return -1;
    }

    fn traverse_once(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let (len, mut i) = (gas.len(), 0);

        while i < len {
            let (mut sum_of_gas, mut sum_of_cost, mut cnt) = (0, 0, 0);

            while cnt < len {
                let j = (i + cnt) % len;
                sum_of_gas += gas[j];
                sum_of_cost += cost[j];
                if sum_of_cost > sum_of_gas {
                    break;
                }
                cnt += 1;
            }

            if cnt == len {
                return i as i32;
            } else {
                i += cnt + 1;
            }
        }

        return -1;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
