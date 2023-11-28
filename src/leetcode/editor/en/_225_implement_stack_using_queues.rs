//Implement a last-in-first-out (LIFO) stack using only two queues. The
//implemented stack should support all the functions of a normal stack (push, top, pop,
//and empty).
//
// Implement the MyStack class:
//
//
// void push(int x) Pushes element x to the top of the stack.
// int pop() Removes the element on the top of the stack and returns it.
// int top() Returns the element on the top of the stack.
// boolean empty() Returns true if the stack is empty, false otherwise.
//
//
// Notes:
//
//
// You must use only standard operations of a queue, which means that only push
//to back, peek/pop from front, size and is empty operations are valid.
// Depending on your language, the queue may not be supported natively. You may
//simulate a queue using a list or deque (double-ended queue) as long as you use
//only a queue's standard operations.
//
//
//
// Example 1:
//
//
//Input
//["MyStack", "push", "push", "top", "pop", "empty"]
//[[], [1], [2], [], [], []]
//Output
//[null, null, null, 2, 2, false]
//
//Explanation
//MyStack myStack = new MyStack();
//myStack.push(1);
//myStack.push(2);
//myStack.top(); // return 2
//myStack.pop(); // return 2
//myStack.empty(); // return False
//
//
//
// Constraints:
//
//
// 1 <= x <= 9
// At most 100 calls will be made to push, pop, top, and empty.
// All the calls to pop and top are valid.
//
//
//
// Follow-up: Can you implement the stack using only one queue?
//
// Related Topics Stack Design Queue 👍 5718 👎 1170

#![allow(dead_code)]
pub struct MyStack2 {
    q1: VecDeque<i32>,
    q2: VecDeque<i32>,
}
impl MyStack2 {
    fn new() -> Self {
        MyStack2 {
            q1: VecDeque::new(),
            q2: VecDeque::new(),
        }
    }

    /// Time Complexity: O(n)
    ///
    /// Space Complexity: O(n)
    fn push(&mut self, x: i32) {
        self.q2.push_back(x);
        while let Some(val) = self.q1.pop_front() {
            self.q2.push_back(val);
        }
        std::mem::swap(&mut self.q1, &mut self.q2);
    }

    /// Time Complexity: O(1)
    ///
    /// Space Complexity: O(1)
    fn pop(&mut self) -> i32 {
        self.q1.pop_front().unwrap()
    }

    /// Time Complexity: O(1)
    ///
    /// Space Complexity: O(1)
    fn top(&mut self) -> i32 {
        *self.q1.front().unwrap()
    }

    /// Time Complexity: O(1)
    ///
    /// Space Complexity: O(1)
    fn empty(&self) -> bool {
        self.q1.is_empty()
    }
}

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::VecDeque;
pub struct MyStack {
    q1: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    fn new() -> Self {
        MyStack {
            q1: VecDeque::new(),
        }
    }

    /// Time Complexity: O(n)
    ///
    /// Space Complexity: O(n)
    fn push(&mut self, x: i32) {
        let mut len = self.q1.len();
        self.q1.push_back(x);
        while len != 0 {
            if let Some(val) = self.q1.pop_front() {
                self.q1.push_back(val);
            }
            len -= 1;
        }
    }

    /// Time Complexity: O(1)
    ///
    /// Space Complexity: O(1)
    fn pop(&mut self) -> i32 {
        self.q1.pop_front().unwrap()
    }

    /// Time Complexity: O(1)
    ///
    /// Space Complexity: O(1)
    fn top(&mut self) -> i32 {
        *self.q1.front().unwrap()
    }

    /// Time Complexity: O(1)
    ///
    /// Space Complexity: O(1)
    fn empty(&self) -> bool {
        self.q1.is_empty()
    }
}

//leetcode submit region end(Prohibit modification and deletion)
