use leetcode_exercise::leetcode::editor::cn::_65_valid_number::Solution;

#[test]
fn valid_number() {
    let s = "0".to_string();
    let res = Solution::is_number(s);
    assert_eq!(res, true);

    let s = "e".to_string();
    let res = Solution::is_number(s);
    assert_eq!(res, false);

    let s = ".".to_string();
    let res = Solution::is_number(s);
    assert_eq!(res, false);

    let s = "0e".to_string();
    let res = Solution::is_number(s);
    assert_eq!(res, false);

    let s = "0..".to_string();
    let res = Solution::is_number(s);
    assert_eq!(res, false);

    let s = ".1.".to_string();
    let res = Solution::is_number(s);
    assert_eq!(res, false);

    let s = "6+1".to_string();
    let res = Solution::is_number(s);
    assert_eq!(res, false);

    let s = "2e0".to_string();
    let res = Solution::is_number(s);
    assert_eq!(res, true);

    let s = "+".to_string();
    let res = Solution::is_number(s);
    assert_eq!(res, false);

    let s = "6e6.5".to_string();
    let res = Solution::is_number(s);
    assert_eq!(res, false);
}
