use leetcode_rust::leetcode::editor::cn::_32_longest_valid_parentheses::Solution;

#[test]
fn longest_valid_parentheses() {
    let s = "(()".to_string();
    let len = Solution::longest_valid_parentheses(s);
    assert_eq!(len, 2);

    let s = ")()())".to_string();
    let len = Solution::longest_valid_parentheses(s);
    assert_eq!(len, 4);
}
