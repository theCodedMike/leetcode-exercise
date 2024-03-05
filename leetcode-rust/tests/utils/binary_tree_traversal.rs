pub mod safe {
    use leetcode_exercise::binary_tree_with_next::safe::Node;
    use std::cell::RefCell;
    use std::rc::Rc;

    pub fn pre_order_recur(root: Option<Rc<RefCell<Node>>>) -> Vec<i32> {
        let mut res = vec![];
        const PRE_ORDER: fn(Option<Rc<RefCell<Node>>>, &mut Vec<i32>) = |root, res| {
            if let Some(curr) = root {
                res.push(curr.borrow().val);
                PRE_ORDER(curr.borrow_mut().left.clone(), res);
                PRE_ORDER(curr.borrow_mut().right.clone(), res);
            }
        };
        PRE_ORDER(root, &mut res);
        res
    }
    pub fn in_order_recur(root: Option<Rc<RefCell<Node>>>) -> Vec<i32> {
        let mut res = vec![];
        const IN_ORDER: fn(Option<Rc<RefCell<Node>>>, &mut Vec<i32>) = |root, res| {
            if let Some(curr) = root {
                IN_ORDER(curr.borrow_mut().left.clone(), res);
                res.push(curr.borrow().val);
                IN_ORDER(curr.borrow_mut().right.clone(), res);
            }
        };
        IN_ORDER(root, &mut res);
        res
    }

    pub fn post_order_recur(root: Option<Rc<RefCell<Node>>>) -> Vec<i32> {
        let mut res = vec![];
        const POST_ORDER: fn(Option<Rc<RefCell<Node>>>, &mut Vec<i32>) = |root, res| {
            if let Some(curr) = root {
                POST_ORDER(curr.borrow_mut().left.clone(), res);
                POST_ORDER(curr.borrow_mut().right.clone(), res);
                res.push(curr.borrow().val);
            }
        };
        POST_ORDER(root, &mut res);
        res
    }
}

pub mod raw_ptr {
    use leetcode_exercise::binary_tree_with_next::raw_ptr::Node;

    pub fn pre_order_recur(root: *mut Node) -> Vec<i32> {
        let mut res = vec![];
        const PRE_ORDER: fn(*mut Node, &mut Vec<i32>) = |root, res| {
            if !root.is_null() {
                unsafe {
                    res.push((*root).val);
                    PRE_ORDER((*root).left, res);
                    PRE_ORDER((*root).right, res);
                }
            }
        };
        PRE_ORDER(root, &mut res);
        res
    }

    pub fn in_order_recur(root: *mut Node) -> Vec<i32> {
        let mut res = vec![];
        const IN_ORDER: fn(*mut Node, &mut Vec<i32>) = |root, res| {
            if !root.is_null() {
                unsafe {
                    IN_ORDER((*root).left, res);
                    res.push((*root).val);
                    IN_ORDER((*root).right, res);
                }
            }
        };
        IN_ORDER(root, &mut res);
        res
    }

    pub fn post_order_recur(root: *mut Node) -> Vec<i32> {
        let mut res = vec![];
        const POST_ORDER: fn(*mut Node, &mut Vec<i32>) = |root, res| {
            if !root.is_null() {
                unsafe {
                    POST_ORDER((*root).left, res);
                    POST_ORDER((*root).right, res);
                    res.push((*root).val);
                }
            }
        };
        POST_ORDER(root, &mut res);
        res
    }
}

pub mod nonnull {
    use leetcode_exercise::binary_tree_with_next::nonnull::Node;
    use std::ptr::NonNull;

    pub fn pre_order_recur(root: Option<NonNull<Node>>) -> Vec<i32> {
        let mut res = vec![];
        const PRE_ORDER: fn(Option<NonNull<Node>>, &mut Vec<i32>) = |root, res| {
            if let Some(curr) = root {
                unsafe {
                    res.push(curr.as_ref().val);
                    PRE_ORDER(curr.as_ref().left, res);
                    PRE_ORDER(curr.as_ref().right, res);
                }
            }
        };
        PRE_ORDER(root, &mut res);
        res
    }

    pub fn in_order_recur(root: Option<NonNull<Node>>) -> Vec<i32> {
        let mut res = vec![];
        const IN_ORDER: fn(Option<NonNull<Node>>, &mut Vec<i32>) = |root, res| {
            if let Some(curr) = root {
                unsafe {
                    IN_ORDER(curr.as_ref().left, res);
                    res.push(curr.as_ref().val);
                    IN_ORDER(curr.as_ref().right, res);
                }
            }
        };
        IN_ORDER(root, &mut res);
        res
    }

    pub fn post_order_recur(root: Option<NonNull<Node>>) -> Vec<i32> {
        let mut res = vec![];
        const POST_ORDER: fn(Option<NonNull<Node>>, &mut Vec<i32>) = |root, res| {
            if let Some(curr) = root {
                unsafe {
                    POST_ORDER(curr.as_ref().left, res);
                    POST_ORDER(curr.as_ref().right, res);
                    res.push(curr.as_ref().val);
                }
            }
        };
        POST_ORDER(root, &mut res);
        res
    }
}
