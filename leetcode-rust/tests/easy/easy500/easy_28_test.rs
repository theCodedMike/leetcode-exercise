use leetcode_exercise::leetcode::editor::cn::_28_find_the_index_of_the_first_occurrence_in_a_string::Solution;

#[test]
fn find_the_index_of_the_first_occurrence_in_a_string_1() {
    let haystack = "sadbutsad".to_string();
    let needle = "sad".to_string();
    let res = Solution::str_str(haystack, needle);
    assert_eq!(res, 0);
}

#[test]
fn find_the_index_of_the_first_occurrence_in_a_string_2() {
    let haystack = "leetcode".to_string();
    let needle = "leeto".to_string();
    let res = Solution::str_str(haystack, needle);
    assert_eq!(res, -1);
}

#[test]
fn find_the_index_of_the_first_occurrence_in_a_string_3() {
    let haystack = "hello".to_string();
    let needle = "ll".to_string();
    let res = Solution::str_str(haystack, needle);
    assert_eq!(res, 2);
}
