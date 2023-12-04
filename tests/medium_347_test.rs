use leetcode_exercise::leetcode::editor::en::_347_top_k_frequent_elements::Solution;

#[test]
fn top_k_frequent_elements_1() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    let res = Solution::top_k_frequent(nums, k);
    assert_eq!(res, [2, 1]); // [1, 2]也可以，与顺序无关
}

#[test]
fn top_k_frequent_elements_2() {
    let nums = vec![1];
    let k = 1;
    let res = Solution::top_k_frequent(nums, k);
    assert_eq!(res, [1]);
}
