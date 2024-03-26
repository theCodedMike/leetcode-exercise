use leetcode_rust::binary_tree::safe::BinaryTree;
use leetcode_rust::leetcode::editor::cn::_968_binary_tree_cameras::Solution;
use leetcode_rust::Build;

#[test]
fn binary_tree_cameras_1() {
    //       0
    //      /
    //     0(camera)
    //    / \
    //   0   0
    let root = BinaryTree::build([Some(0), Some(0), None, Some(0), Some(0)].as_slice());
    assert_eq!(Solution::min_camera_cover(root), 1);
}

#[test]
fn binary_tree_cameras_2() {
    //         0
    //        /
    //       0(camera)
    //      /
    //     0
    //    /
    //   0(camera)
    //    \
    //     0
    let root = BinaryTree::build(
        [
            Some(0),
            Some(0),
            None,
            Some(0),
            None,
            Some(0),
            None,
            None,
            Some(0),
        ]
        .as_slice(),
    );
    assert_eq!(Solution::min_camera_cover(root), 2);
}

#[test]
fn binary_tree_cameras_3() {
    //    0(camera)
    let root = BinaryTree::build([Some(0)].as_slice());
    assert_eq!(Solution::min_camera_cover(root), 1);
}

#[test]
fn binary_tree_cameras_4() {
    //     0(camera)
    //    / \
    //   0   0
    let root = BinaryTree::build([Some(0), Some(0), Some(0)].as_slice());
    assert_eq!(Solution::min_camera_cover(root), 1);
}
