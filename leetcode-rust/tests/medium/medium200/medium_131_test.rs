use leetcode_rust::leetcode::editor::cn::_131_palindrome_partitioning::Solution;
use std::collections::HashSet;

#[test]
fn palindrome_partitioning_1() {
    let s = "aab".to_string();
    let res = Solution::partition(s);

    assert_eq!(res.len(), 2);
    let set = HashSet::from([
        vec!["a".to_string(), "a".to_string(), "b".to_string()],
        vec!["aa".to_string(), "b".to_string()],
    ]);
    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}

#[test]
fn palindrome_partitioning_2() {
    let s = "a".to_string();
    let res = Solution::partition(s);

    assert_eq!(res.len(), 1);
    let set = HashSet::from([vec!["a".to_string()]]);
    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}
