//Design a stack that supports push, pop, top, and retrieving the minimum
//element in constant time.
//
// Implement the MinStack class:
//
//
// MinStack() initializes the stack object.
// void push(int val) pushes the element val onto the stack.
// void pop() removes the element on the top of the stack.
// int top() gets the top element of the stack.
// int getMin() retrieves the minimum element in the stack.
//
//
// You must implement a solution with O(1) time complexity for each function.
//
//
// Example 1:
//
//
//Input
//["MinStack","push","push","push","getMin","pop","top","getMin"]
//[[],[-2],[0],[-3],[],[],[],[]]
//
//Output
//[null,null,null,null,-3,null,0,-2]
//
//Explanation
//MinStack minStack = new MinStack();
//minStack.push(-2);
//minStack.push(0);
//minStack.push(-3);
//minStack.getMin(); // return -3
//minStack.pop();
//minStack.top();    // return 0
//minStack.getMin(); // return -2
//
//
//
// Constraints:
//
//
// -2³¹ <= val <= 2³¹ - 1
// Methods pop, top and getMin operations will always be called on non-empty
//stacks.
// At most 3 * 10⁴ calls will be made to push, pop, top, and getMin.
//
//
// Related Topics Stack Design 👍 12846 👎 783

#![allow(dead_code)]

//leetcode submit region begin(Prohibit modification and deletion)

pub struct MinStack {
    data: Vec<(i32, i32)>,
    curr_min: Option<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    pub fn new() -> Self {
        MinStack {
            data: vec![],
            curr_min: None,
        }
    }

    pub fn push(&mut self, val: i32) {
        match self.curr_min {
            None => {
                self.data.push((val, val));
                self.curr_min = Some(val);
            }
            Some(old) => {
                let min = if val < old {
                    self.curr_min.replace(val);
                    val
                } else {
                    old
                };
                self.data.push((val, min));
            }
        }
    }

    pub fn pop(&mut self) {
        let _ = self.data.pop();
        if self.data.is_empty() {
            self.curr_min = None;
        } else {
            self.curr_min.replace(self.get_min());
        }
    }

    pub fn top(&self) -> i32 {
        (*self.data.last().unwrap_or(&(0, 0))).0
    }

    pub fn get_min(&self) -> i32 {
        (*self.data.last().unwrap_or(&(0, 0))).1
    }
}

//leetcode submit region end(Prohibit modification and deletion)
