use leetcode_exercise::leetcode::editor::cn::_559_maximum_depth_of_n_ary_tree::Solution;
use leetcode_exercise::n_ary_tree::Node;

#[test]
fn maximum_depth_of_n_ary_tree_1() {
    //        1
    //     /  |  \
    //    3   2   4
    //   / \
    //  5   6
    let root = Node::new_with_children(
        1,
        vec![
            Node::new_with_children(3, vec![Node::new(5), Node::new(6)]),
            Node::new(2),
            Node::new(4),
        ],
    );

    assert_eq!(Solution::max_depth(root), 3);
}

#[test]
fn maximum_depth_of_n_ary_tree_2() {
    //        1
    //   /   / \   \
    //  2   3   4    5
    //     / \  |   / \
    //    6  7  8  9   10
    //       |  |  |
    //      11 12 13
    //      |
    //      14
    let root = Node::new_with_children(
        1,
        vec![
            Node::new(2),
            Node::new_with_children(
                3,
                vec![
                    Node::new(6),
                    Node::new_with_children(
                        7,
                        vec![Node::new_with_children(11, vec![Node::new(14)])],
                    ),
                ],
            ),
            Node::new_with_children(4, vec![Node::new_with_children(8, vec![Node::new(12)])]),
            Node::new_with_children(
                5,
                vec![
                    Node::new_with_children(9, vec![Node::new(13)]),
                    Node::new(10),
                ],
            ),
        ],
    );

    assert_eq!(Solution::max_depth(root), 5);
}
