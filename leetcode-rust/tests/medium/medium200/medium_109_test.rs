use leetcode_exercise::binary_tree::safe::BinaryTree;
use leetcode_exercise::leetcode::editor::cn::_109_convert_sorted_list_to_binary_search_tree::Solution;
use leetcode_exercise::linked_list::singly_linked_list::safe::LinkedList;
use leetcode_exercise::{Build, Traverse};

#[test]
fn convert_sorted_list_to_binary_search_tree_1() {
    //  -10 -> -3 -> 0 -> 5 -> 9
    //               ↓
    //
    //              0
    //             / \
    //           -3   9
    //           /   /
    //         -10  5
    let head = LinkedList::build(&[-10, -3, 0, 5, 9]);
    let root = Solution::sorted_list_to_bst(head);

    assert_eq!(
        BinaryTree::pre_order_recur(root.clone()),
        [0, -3, -10, 9, 5]
    );
    assert_eq!(BinaryTree::in_order_recur(root.clone()), [-10, -3, 0, 5, 9]);
    assert_eq!(
        BinaryTree::post_order_recur(root.clone()),
        [-10, -3, 5, 9, 0]
    );
    assert_eq!(BinaryTree::level_order(root), [0, -3, 9, -10, 5]);
}

#[test]
fn convert_sorted_list_to_binary_search_tree_2() {
    let head = LinkedList::build(&[]);
    let root = Solution::sorted_list_to_bst(head);
    assert_eq!(root, None);
}
