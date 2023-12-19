use leetcode_exercise::leetcode::editor::en::_104_maximum_depth_of_binary_tree::{
    Solution, TreeNode,
};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn maximum_depth_of_binary_tree() {
    let node_7 = TreeNode::new(7);
    let node_15 = TreeNode::new(15);
    let node_9 = TreeNode::new(9);
    let node_20 = TreeNode::new2(20, node_15, node_7);
    let node_3 = TreeNode::new2(3, node_9, node_20);
    let root = Some(Rc::new(RefCell::new(node_3)));

    let depth = Solution::max_depth(root);
    assert_eq!(depth, 3);
}
