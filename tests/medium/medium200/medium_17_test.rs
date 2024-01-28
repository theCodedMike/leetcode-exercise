use leetcode_exercise::leetcode::editor::cn::_17_letter_combinations_of_a_phone_number::Solution;
use std::collections::HashSet;

#[test]
fn letter_combinations_of_a_phone_number_1() {
    let digits = "23".to_string();
    let res = Solution::letter_combinations(digits);

    assert_eq!(res.len(), 9);
    let set = HashSet::from([
        "ad".to_string(),
        "ae".to_string(),
        "af".to_string(),
        "bd".to_string(),
        "be".to_string(),
        "bf".to_string(),
        "cd".to_string(),
        "ce".to_string(),
        "cf".to_string(),
    ]);
    for item in res {
        assert_eq!(set.contains(&item), true)
    }
}

#[test]
fn letter_combinations_of_a_phone_number_2() {
    let digits = "".to_string();
    let res = Solution::letter_combinations(digits);

    assert_eq!(res.is_empty(), true);
}

#[test]
fn letter_combinations_of_a_phone_number_3() {
    let digits = "2".to_string();
    let res = Solution::letter_combinations(digits);

    assert_eq!(res.len(), 3);
    let set = HashSet::from(["a".to_string(), "b".to_string(), "c".to_string()]);
    for item in res {
        assert_eq!(set.contains(&item), true)
    }
}
