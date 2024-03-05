use leetcode_exercise::leetcode::editor::cn::_173_binary_search_tree_iterator::{
    BSTIterator, TreeNode,
};

#[test]
fn binary_search_tree_iterator() {
    //     7
    //    / \
    //   3  15
    //     /  \
    //    9    20
    let root = TreeNode::new2(
        7,
        TreeNode::new2(3, None, None),
        TreeNode::new2(
            15,
            TreeNode::new2(9, None, None),
            TreeNode::new2(20, None, None),
        ),
    );
    let mut iterator = BSTIterator::new(root);
    while iterator.has_next() {
        println!("{}", iterator.next());
    }
}
