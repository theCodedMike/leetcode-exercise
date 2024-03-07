use leetcode_rust::leetcode::editor::cn::_1047_remove_all_adjacent_duplicates_in_string::Solution;

#[test]
fn remove_all_adjacent_duplicates_in_string_1() {
    let s = "abbaca".to_string();
    let res = Solution::remove_duplicates(s);
    assert_eq!(res, "ca");
}

#[test]
fn remove_all_adjacent_duplicates_in_string_2() {
    let s = "azxxzy".to_string();
    let res = Solution::remove_duplicates(s);
    assert_eq!(res, "ay");
}
