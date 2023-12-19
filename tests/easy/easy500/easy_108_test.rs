use leetcode_exercise::leetcode::editor::en::_108_convert_sorted_array_to_binary_search_tree::Solution;

#[test]
fn convert_sorted_array_to_binary_search_tree() {
    let nums = vec![-10, -3, 0, 5, 9];
    let root = Solution::sorted_array_to_bst(nums);
    println!("{:#?}", root);
}
