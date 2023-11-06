use leetcode_exercise::leetcode::editor::en::_438_find_all_anagrams_in_a_string::Solution;

#[test]
fn find_all_anagrams_in_a_string() {
    let s = "cbaebabacd".to_string();
    let p = "abc".to_string();
    let res = Solution::find_anagrams(s, p);
    assert_eq!(res, [0, 6]);

    let s = "abab".to_string();
    let p = "ab".to_string();
    let res = Solution::find_anagrams(s, p);
    assert_eq!(res, [0, 1, 2]);
}
