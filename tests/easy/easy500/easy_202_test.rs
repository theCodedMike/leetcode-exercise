use leetcode_exercise::leetcode::editor::en::_202_happy_number::Solution;
#[test]
fn happy_number_test() {
    let n = 19;
    let res = Solution::is_happy(n);
    assert_eq!(res, true);

    let n = 2;
    let res = Solution::is_happy(n);
    assert_eq!(res, false);
}
