use leetcode_rust::leetcode::editor::cn::_844_backspace_string_compare::Solution;

#[test]
fn backspace_string_compare() {
    let s = "ab#c".to_string();
    let t = "ad#c".to_string();
    let res = Solution::backspace_compare(s, t);
    assert_eq!(res, true);

    let s = "ab##".to_string();
    let t = "c#d#".to_string();
    let res = Solution::backspace_compare(s, t);
    assert_eq!(res, true);

    let s = "a#c".to_string();
    let t = "b".to_string();
    let res = Solution::backspace_compare(s, t);
    assert_eq!(res, false);

    let s = "xywrrmp".to_string();
    let t = "xywrrmu#p".to_string();
    let res = Solution::backspace_compare(s, t);
    assert_eq!(res, true);

    let s = "bxj##tw".to_string();
    let t = "bxj###tw".to_string();
    let res = Solution::backspace_compare(s, t);
    assert_eq!(res, false);

    let s = "y#fo##f".to_string();
    let t = "y#fx#o##f".to_string();
    let res = Solution::backspace_compare(s, t);
    assert_eq!(res, true);
}
