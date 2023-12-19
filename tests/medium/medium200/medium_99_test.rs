use leetcode_exercise::leetcode::editor::en::_99_recover_binary_search_tree::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn recover_binary_search_tree() {
    let node_2 = TreeNode::new(2, None, None);
    let node_3 = TreeNode::new(3, None, Some(node_2));
    let node_1 = TreeNode::new(1, Some(node_3), None);
    let mut root = Some(Rc::new(RefCell::new(node_1)));

    Solution::recover_tree(&mut root);
}
