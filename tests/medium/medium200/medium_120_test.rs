use leetcode_exercise::leetcode::editor::cn::_120_triangle::Solution;

#[test]
fn triangle() {
    let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
    let res = Solution::minimum_total(triangle);
    assert_eq!(res, 11);
}
