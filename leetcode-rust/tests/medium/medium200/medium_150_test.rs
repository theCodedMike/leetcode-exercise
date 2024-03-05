use leetcode_exercise::leetcode::editor::cn::_150_evaluate_reverse_polish_notation::Solution;

#[test]
fn evaluate_reverse_polish_notation_1() {
    let tokens = vec![
        "2".to_string(),
        "1".to_string(),
        "+".to_string(),
        "3".to_string(),
        "*".to_string(),
    ];
    let res = Solution::eval_rpn(tokens);
    assert_eq!(res, 9);
}

#[test]
fn evaluate_reverse_polish_notation_2() {
    let tokens = vec![
        "4".to_string(),
        "13".to_string(),
        "5".to_string(),
        "/".to_string(),
        "+".to_string(),
    ];
    let res = Solution::eval_rpn(tokens);
    assert_eq!(res, 6);
}
#[test]
fn evaluate_reverse_polish_notation_3() {
    let tokens = vec![
        "10".to_string(),
        "6".to_string(),
        "9".to_string(),
        "3".to_string(),
        "+".to_string(),
        "-11".to_string(),
        "*".to_string(),
        "/".to_string(),
        "*".to_string(),
        "17".to_string(),
        "+".to_string(),
        "5".to_string(),
        "+".to_string(),
    ];
    let res = Solution::eval_rpn(tokens);
    assert_eq!(res, 22);
}
