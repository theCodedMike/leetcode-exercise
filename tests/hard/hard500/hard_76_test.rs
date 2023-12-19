use leetcode_exercise::leetcode::editor::en::_76_minimum_window_substring::Solution;

#[test]
fn minimum_window_substring() {
    let s = "ADOBECODEBANC".to_string();
    let t = "ABC".to_string();
    let res = Solution::min_window(s, t);
    assert_eq!(res, "BANC");

    let s = "a".to_string();
    let t = "a".to_string();
    let res = Solution::min_window(s, t);
    assert_eq!(res, "a");

    let s = "a".to_string();
    let t = "aa".to_string();
    let res = Solution::min_window(s, t);
    assert_eq!(res, "");

    let s = "bbaa".to_string();
    let t = "aba".to_string();
    let res = Solution::min_window(s, t);
    assert_eq!(res, "baa");

    let s = "aaaaaaaaaaaabbbbbcdd".to_string();
    let t = "abcdd".to_string();
    let res = Solution::min_window(s, t);
    assert_eq!(res, "abbbbbcdd");
}
