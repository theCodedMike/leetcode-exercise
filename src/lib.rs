pub mod leetcode;

pub trait Traverse {
    type NodeType;
    type ElemType;
    /// 递归前序遍历
    fn pre_order_recur(root: Self::NodeType) -> Vec<Self::ElemType>;
    /// 递归中序遍历
    fn in_order_recur(root: Self::NodeType) -> Vec<Self::ElemType>;
    /// 递归后序遍历
    fn post_order_recur(root: Self::NodeType) -> Vec<Self::ElemType>;

    /// 迭代前序遍历
    fn pre_order_iter(root: Self::NodeType) -> Vec<Self::ElemType>;
    /// 迭代中序遍历
    fn in_order_iter(root: Self::NodeType) -> Vec<Self::ElemType>;
    /// 迭代后序遍历
    fn post_order_iter(root: Self::NodeType) -> Vec<Self::ElemType>;

    /// 层序遍历
    fn level_order(root: Self::NodeType) -> Vec<Self::ElemType>;
}

pub trait Build {
    type ElemType;
    type NodeType;

    fn build(elems: &[Self::ElemType]) -> Self::NodeType;
}

pub mod binary_tree {
    pub mod safe {
        use crate::{Build, Traverse};
        use std::cell::RefCell;
        use std::collections::VecDeque;
        use std::rc::Rc;

        pub struct BinaryTree;
        impl Build for BinaryTree {
            type ElemType = Option<i32>;
            type NodeType = Option<Rc<RefCell<TreeNode>>>;
            fn build(elems: &[Self::ElemType]) -> Self::NodeType {
                if elems.is_empty() || elems[0].is_none() {
                    return None;
                }

                let root = TreeNode::new2(elems[0].unwrap_or_default());
                let mut queue = VecDeque::from([root.clone()]);
                let (mut idx, len) = (0, elems.len());

                while let Some(curr) = queue.pop_front() {
                    if let Some(curr) = curr {
                        idx += 1;
                        if idx == len {
                            break;
                        }
                        if let Some(val) = elems[idx] {
                            let left = TreeNode::new2(val);
                            curr.borrow_mut().left = left.clone();
                            queue.push_back(left)
                        }
                        idx += 1;
                        if idx == len {
                            break;
                        }
                        if let Some(val) = elems[idx] {
                            let right = TreeNode::new2(val);
                            curr.borrow_mut().right = right.clone();
                            queue.push_back(right);
                        }
                    }
                }

                root
            }
        }
        impl Traverse for BinaryTree {
            type NodeType = Option<Rc<RefCell<TreeNode>>>;
            type ElemType = i32;

            fn pre_order_recur(root: Self::NodeType) -> Vec<Self::ElemType> {
                let mut res = vec![];
                const PRE_ORDER: fn(Option<Rc<RefCell<TreeNode>>>, &mut Vec<i32>) = |root, res| {
                    if let Some(curr) = root {
                        res.push(curr.borrow().val);
                        PRE_ORDER(curr.borrow().left.clone(), res);
                        PRE_ORDER(curr.borrow().right.clone(), res);
                    }
                };
                PRE_ORDER(root, &mut res);
                res
            }

            fn in_order_recur(root: Self::NodeType) -> Vec<Self::ElemType> {
                let mut res = vec![];
                const IN_ORDER: fn(Option<Rc<RefCell<TreeNode>>>, &mut Vec<i32>) = |root, res| {
                    if let Some(curr) = root {
                        IN_ORDER(curr.borrow().left.clone(), res);
                        res.push(curr.borrow().val);
                        IN_ORDER(curr.borrow().right.clone(), res);
                    }
                };
                IN_ORDER(root, &mut res);
                res
            }

            fn post_order_recur(root: Self::NodeType) -> Vec<Self::ElemType> {
                let mut res = vec![];
                const POST_ORDER: fn(Option<Rc<RefCell<TreeNode>>>, &mut Vec<i32>) = |root, res| {
                    if let Some(curr) = root {
                        POST_ORDER(curr.borrow().left.clone(), res);
                        POST_ORDER(curr.borrow().right.clone(), res);
                        res.push(curr.borrow().val);
                    }
                };
                POST_ORDER(root, &mut res);
                res
            }
            fn pre_order_iter(root: Self::NodeType) -> Vec<Self::ElemType> {
                let mut res = vec![];
                if let Some(root) = root {
                    let mut stack = vec![Ok(root)];
                    while let Some(curr) = stack.pop() {
                        match curr {
                            Ok(node) => {
                                if let Some(right) = node.borrow().right.clone() {
                                    stack.push(Ok(right));
                                }
                                if let Some(left) = node.borrow().left.clone() {
                                    stack.push(Ok(left));
                                }
                                stack.push(Err(node.borrow().val));
                            }
                            Err(val) => res.push(val),
                        }
                    }
                }
                res
            }

            fn in_order_iter(root: Self::NodeType) -> Vec<Self::ElemType> {
                let mut res = vec![];
                if let Some(root) = root {
                    let mut stack = vec![Ok(root)];
                    while let Some(curr) = stack.pop() {
                        match curr {
                            Ok(node) => {
                                if let Some(right) = node.borrow().right.clone() {
                                    stack.push(Ok(right));
                                }
                                stack.push(Err(node.borrow().val));
                                if let Some(left) = node.borrow().left.clone() {
                                    stack.push(Ok(left));
                                }
                            }
                            Err(val) => res.push(val),
                        }
                    }
                }
                res
            }

            fn post_order_iter(root: Self::NodeType) -> Vec<Self::ElemType> {
                let mut res = vec![];
                if let Some(root) = root {
                    let mut stack = vec![Ok(root)];
                    while let Some(curr) = stack.pop() {
                        match curr {
                            Ok(node) => {
                                stack.push(Err(node.borrow().val));
                                if let Some(right) = node.borrow().right.clone() {
                                    stack.push(Ok(right));
                                }
                                if let Some(left) = node.borrow().left.clone() {
                                    stack.push(Ok(left));
                                }
                            }
                            Err(val) => res.push(val),
                        }
                    }
                }
                res
            }

            fn level_order(root: Self::NodeType) -> Vec<Self::ElemType> {
                let mut res = vec![];
                if let Some(root) = root {
                    let mut queue = VecDeque::from([root]);
                    while let Some(curr) = queue.pop_front() {
                        res.push(curr.borrow().val);
                        if let Some(left) = curr.borrow().left.clone() {
                            queue.push_back(left);
                        }
                        if let Some(right) = curr.borrow().right.clone() {
                            queue.push_back(right);
                        }
                    }
                }
                res
            }
        }
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
        pub struct TreeNode {
            pub val: i32,
            pub left: Option<Rc<RefCell<TreeNode>>>,
            pub right: Option<Rc<RefCell<TreeNode>>>,
        }

        impl TreeNode {
            ///
            /// Node with no children
            ///
            pub fn new(val: i32) -> Self {
                TreeNode {
                    val,
                    left: None,
                    right: None,
                }
            }

            ///
            /// Node with no children
            ///
            pub fn new2(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
                Self::new_with_children(val, None, None)
            }

            ///
            /// Node with left child
            ///
            pub fn new_with_left(
                val: i32,
                left: Option<Rc<RefCell<TreeNode>>>,
            ) -> Option<Rc<RefCell<TreeNode>>> {
                Self::new_with_children(val, left, None)
            }

            ///
            /// Node with right child
            ///
            pub fn new_with_right(
                val: i32,
                right: Option<Rc<RefCell<TreeNode>>>,
            ) -> Option<Rc<RefCell<TreeNode>>> {
                Self::new_with_children(val, None, right)
            }

            ///
            /// Node with children
            ///
            pub fn new_with_children(
                val: i32,
                left: Option<Rc<RefCell<TreeNode>>>,
                right: Option<Rc<RefCell<TreeNode>>>,
            ) -> Option<Rc<RefCell<TreeNode>>> {
                Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
            }
        }
    }
    pub mod raw_ptr {}
    pub mod nonnull {}
}

pub mod n_ary_tree {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug, PartialEq, Eq)]
    pub struct Node {
        pub val: i32,
        pub children: Option<Vec<Option<Rc<RefCell<Node>>>>>,
    }
    impl Node {
        ///
        /// Node with no children
        ///
        pub fn new(val: i32) -> Option<Rc<RefCell<Node>>> {
            Some(Rc::new(RefCell::new(Node {
                val,
                children: None,
            })))
        }

        ///
        /// Node with children
        ///
        pub fn new_with_children(
            val: i32,
            children: Vec<Option<Rc<RefCell<Node>>>>,
        ) -> Option<Rc<RefCell<Node>>> {
            Some(Rc::new(RefCell::new(Node {
                val,
                children: if children.is_empty() {
                    None
                } else {
                    Some(children)
                },
            })))
        }
    }
}

pub mod binary_tree_with_next {
    pub mod safe {
        use std::cell::RefCell;
        use std::rc::Rc;

        #[derive(Debug, PartialEq, Eq)]
        pub struct Node {
            pub val: i32,
            pub left: Option<Rc<RefCell<Node>>>,
            pub right: Option<Rc<RefCell<Node>>>,
            pub next: Option<Rc<RefCell<Node>>>,
        }
        impl Node {
            pub fn new(val: i32) -> Option<Rc<RefCell<Node>>> {
                Self::new_with_children(val, None, None)
            }

            pub fn new_with_left(
                val: i32,
                left: Option<Rc<RefCell<Node>>>,
            ) -> Option<Rc<RefCell<Node>>> {
                Self::new_with_children(val, left, None)
            }

            pub fn new_with_right(
                val: i32,
                right: Option<Rc<RefCell<Node>>>,
            ) -> Option<Rc<RefCell<Node>>> {
                Self::new_with_children(val, None, right)
            }

            pub fn new_with_children(
                val: i32,
                left: Option<Rc<RefCell<Node>>>,
                right: Option<Rc<RefCell<Node>>>,
            ) -> Option<Rc<RefCell<Node>>> {
                Some(Rc::new(RefCell::new(Node {
                    val,
                    left,
                    right,
                    next: None,
                })))
            }
        }
    }

    pub mod raw_ptr {
        use std::ptr::null_mut;

        #[derive(Debug, PartialEq, Eq)]
        pub struct Node {
            pub val: i32,
            pub left: *mut Node,
            pub right: *mut Node,
            pub next: *mut Node,
        }

        impl Node {
            pub fn new(val: i32) -> *mut Node {
                Self::new_with_children(val, null_mut(), null_mut())
            }
            pub fn new_with_left(val: i32, left: *mut Node) -> *mut Node {
                Self::new_with_children(val, left, null_mut())
            }
            pub fn new_with_right(val: i32, right: *mut Node) -> *mut Node {
                Self::new_with_children(val, null_mut(), right)
            }
            pub fn new_with_children(val: i32, left: *mut Node, right: *mut Node) -> *mut Node {
                Box::into_raw(Box::new(Node {
                    val,
                    left,
                    right,
                    next: null_mut(),
                }))
            }
        }
    }

    pub mod nonnull {
        use std::ptr::NonNull;

        #[derive(Debug, PartialEq, Eq)]
        pub struct Node {
            pub val: i32,
            pub left: Option<NonNull<Node>>,
            pub right: Option<NonNull<Node>>,
            pub next: Option<NonNull<Node>>,
        }
        impl Node {
            pub fn new(val: i32) -> Option<NonNull<Node>> {
                Self::new_with_children(val, None, None)
            }
            pub fn new_with_left(val: i32, left: Option<NonNull<Node>>) -> Option<NonNull<Node>> {
                Self::new_with_children(val, left, None)
            }
            pub fn new_with_right(val: i32, right: Option<NonNull<Node>>) -> Option<NonNull<Node>> {
                Self::new_with_children(val, None, right)
            }
            pub fn new_with_children(
                val: i32,
                left: Option<NonNull<Node>>,
                right: Option<NonNull<Node>>,
            ) -> Option<NonNull<Node>> {
                NonNull::new(Box::into_raw(Box::new(Node {
                    val,
                    left,
                    right,
                    next: None,
                })))
            }
        }
    }
}

pub mod linked_list {
    pub mod singly_linked_list {
        pub mod safe {
            use crate::Build;

            pub struct LinkedList;
            impl Build for LinkedList {
                type ElemType = i32;
                type NodeType = Option<Box<ListNode>>;

                fn build(elems: &[Self::ElemType]) -> Self::NodeType {
                    let mut dummy = ListNode::new(0);

                    for val in elems.into_iter().rev() {
                        dummy.next = ListNode::new_with_next(*val, dummy.next.take());
                    }

                    dummy.next.take()
                }
            }

            #[derive(Debug, PartialEq, Eq)]
            pub struct ListNode {
                pub val: i32,
                pub next: Option<Box<ListNode>>,
            }

            impl ListNode {
                pub fn new(val: i32) -> Self {
                    ListNode { val, next: None }
                }
                pub fn new2(val: i32) -> Option<Box<ListNode>> {
                    Self::new_with_next(val, None)
                }
                pub fn new_with_next(
                    val: i32,
                    next: Option<Box<ListNode>>,
                ) -> Option<Box<ListNode>> {
                    Some(Box::new(ListNode { val, next }))
                }
            }
        }
    }
}
