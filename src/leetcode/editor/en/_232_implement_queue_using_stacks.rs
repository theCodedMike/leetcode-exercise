//Implement a first in first out (FIFO) queue using only two stacks. The
//implemented queue should support all the functions of a normal queue (push, peek, pop,
//and empty).
//
// Implement the MyQueue class:
//
//
// void push(int x) Pushes element x to the back of the queue.
// int pop() Removes the element from the front of the queue and returns it.
// int peek() Returns the element at the front of the queue.
// boolean empty() Returns true if the queue is empty, false otherwise.
//
//
// Notes:
//
//
// You must use only standard operations of a stack, which means only push to
//top, peek/pop from top, size, and is empty operations are valid.
// Depending on your language, the stack may not be supported natively. You may
//simulate a stack using a list or deque (double-ended queue) as long as you use
//only a stack's standard operations.
//
//
//
// Example 1:
//
//
//Input
//["MyQueue", "push", "push", "peek", "pop", "empty"]
//[[], [1], [2], [], [], []]
//Output
//[null, null, null, 1, 1, false]
//
//Explanation
//MyQueue myQueue = new MyQueue();
//myQueue.push(1); // queue is: [1]
//myQueue.push(2); // queue is: [1, 2] (leftmost is front of the queue)
//myQueue.peek(); // return 1
//myQueue.pop(); // return 1, queue is [2]
//myQueue.empty(); // return false
//
//
//
// Constraints:
//
//
// 1 <= x <= 9
// At most 100 calls will be made to push, pop, peek, and empty.
// All the calls to pop and peek are valid.
//
//
//
// Follow-up: Can you implement the queue such that each operation is amortized
//O(1) time complexity? In other words, performing n operations will take overall
//O(n) time even if one of those operations may take longer.
//
// Related Topics Stack Design Queue 👍 6835 👎 389

#![allow(dead_code)]

//leetcode submit region begin(Prohibit modification and deletion)
struct MyQueue {
    s1: Vec<i32>,
    s2: Vec<i32>,
}

/// 这种解法带有优化，性能更好
impl MyQueue {
    fn new() -> Self {
        MyQueue {
            s1: vec![],
            s2: vec![],
        }
    }

    /// Time Complexity: O(1)
    ///
    /// Space Complexity: O(n)
    fn push(&mut self, x: i32) {
        self.s1.push(x);
    }

    /// Time Complexity: Amortized O(1), Worst-case O(n)
    ///
    /// Space Complexity: O(1)
    fn pop(&mut self) -> i32 {
        if self.s2.is_empty() {
            while let Some(val) = self.s1.pop() {
                self.s2.push(val);
            }
        }

        self.s2.pop().unwrap()
    }

    /// Time Complexity: O(1)
    ///
    /// Space Complexity: O(1)
    fn peek(&mut self) -> i32 {
        if self.s2.is_empty() {
            *self.s1.first().unwrap()
        } else {
            *self.s2.last().unwrap()
        }
    }

    /// Time Complexity: O(1)
    ///
    /// Space Complexity: O(1)
    fn empty(&self) -> bool {
        self.s1.is_empty() && self.s2.is_empty()
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    /// Time Complexity: O(n)
    ///
    /// Space Complexity: O(n)
    fn push2(&mut self, x: i32) {
        while let Some(val) = self.s1.pop() {
            self.s2.push(val);
        }
        self.s1.push(x);
        while let Some(val) = self.s2.pop() {
            self.s1.push(val);
        }
    }

    /// Time Complexity: O(1)
    ///
    /// Space Complexity: O(1)
    fn pop2(&mut self) -> i32 {
        self.s1.pop().unwrap()
    }

    /// Time Complexity: O(1)
    ///
    /// Space Complexity: O(1)
    fn peek2(&mut self) -> i32 {
        *self.s1.last().unwrap()
    }

    /// Time Complexity: O(1)
    ///
    /// Space Complexity: O(1)
    fn empty2(&self) -> bool {
        self.s1.is_empty()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
