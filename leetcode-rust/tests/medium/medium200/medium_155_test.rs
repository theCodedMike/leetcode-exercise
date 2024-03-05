use leetcode_exercise::leetcode::editor::cn::_155_min_stack::MinStack;

#[test]
fn min_stack() {
    // 测试用例:["MinStack","push","push","push","top","pop","getMin","pop","getMin","pop","push","top","getMin","push","top","getMin","pop","getMin"]
    // 			[[],[2147483646],[2147483646],[2147483647],[],[],[],[],[],[],[2147483647],[],[],[-2147483648],[],[],[],[]]
    let mut stack = MinStack::new();
    stack.push(2147483646);
    stack.push(2147483646);
    stack.push(2147483647);
    stack.top();
    stack.pop();
    stack.get_min();
    stack.pop();
    stack.get_min();
    stack.pop();
    stack.push(2147483647);
    stack.top();
    stack.get_min();
    stack.push(-2147483648);
    stack.top();
    stack.get_min();
    stack.pop();
    stack.get_min();
}
