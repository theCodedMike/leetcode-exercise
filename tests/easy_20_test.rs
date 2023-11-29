use leetcode_exercise::leetcode::editor::en::_20_valid_parentheses::Solution;

#[test]
fn valid_parentheses_1() {
    let s = "()".to_string();
    let res = Solution::is_valid(s);
    assert_eq!(res, true);
}

#[test]
fn valid_parentheses_2() {
    let s = "()[]{}".to_string();
    let res = Solution::is_valid(s);
    assert_eq!(res, true);
}

#[test]
fn valid_parentheses_3() {
    let s = "(]".to_string();
    let res = Solution::is_valid(s);
    assert_eq!(res, false);
}

#[test]
fn valid_parentheses_4() {
    let s = "[".to_string();
    let res = Solution::is_valid(s);
    assert_eq!(res, false);
}
