use leetcode_rust::leetcode::editor::cn::_3_longest_substring_without_repeating_characters::Solution;

#[test]
fn longest_substring_without_repeating_characters() {
    let s = "abcabcbb".to_string();
    let res = Solution::length_of_longest_substring(s);
    assert_eq!(res, 3);
}
