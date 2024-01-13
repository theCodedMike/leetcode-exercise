use leetcode_exercise::leetcode::editor::cn::_904_fruit_into_baskets::Solution;

#[test]
fn fruit_into_baskets() {
    let fruits = vec![1, 2, 1];
    let res = Solution::total_fruit(fruits);
    assert_eq!(res, 3);

    let fruits = vec![0, 1, 2, 2];
    let res = Solution::total_fruit(fruits);
    assert_eq!(res, 3);

    let fruits = vec![1, 2, 3, 2, 2];
    let res = Solution::total_fruit(fruits);
    assert_eq!(res, 4);

    let fruits = vec![0];
    let res = Solution::total_fruit(fruits);
    assert_eq!(res, 1);

    let fruits = vec![1, 1];
    let res = Solution::total_fruit(fruits);
    assert_eq!(res, 2);
}
