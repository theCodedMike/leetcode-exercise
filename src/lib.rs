pub mod leetcode;

pub mod binary_tree {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug, PartialEq, Eq)]
    pub struct TreeNode {
        pub val: i32,
        pub left: Option<Rc<RefCell<TreeNode>>>,
        pub right: Option<Rc<RefCell<TreeNode>>>,
    }

    impl TreeNode {
        ///
        /// Node with no children
        ///
        pub fn new(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
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
