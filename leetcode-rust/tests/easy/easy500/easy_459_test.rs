use leetcode_rust::leetcode::editor::cn::_459_repeated_substring_pattern::Solution;

#[test]
fn repeated_substring_pattern_1() {
    let s = "abab".to_string();
    let res = Solution::repeated_substring_pattern(s);
    assert_eq!(res, true);
}

#[test]
fn repeated_substring_pattern_2() {
    let s = "aba".to_string();
    let res = Solution::repeated_substring_pattern(s);
    assert_eq!(res, false);
}

#[test]
fn repeated_substring_pattern_3() {
    let s = "abcabcabcabc".to_string();
    let res = Solution::repeated_substring_pattern(s);
    assert_eq!(res, true);
}

#[test]
fn repeated_substring_pattern_4() {
    let s = "a".to_string();
    let res = Solution::repeated_substring_pattern(s);
    assert_eq!(res, false);
}

#[test]
fn repeated_substring_pattern_5() {
    let s = "ababba".to_string();
    let res = Solution::repeated_substring_pattern(s);
    assert_eq!(res, false);
}

#[test]
fn repeated_substring_pattern_6() {
    let s = "abaababaab".to_string();
    let res = Solution::repeated_substring_pattern(s);
    assert_eq!(res, true);
}

#[test]
fn repeated_substring_pattern_7() {
    let s = "aabaaba".to_string();
    let res = Solution::repeated_substring_pattern(s);
    assert_eq!(res, false);
}
