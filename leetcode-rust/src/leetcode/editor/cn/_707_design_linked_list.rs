//Design your implementation of the linked list. You can choose to use a singly
//or doubly linked list. A node in a singly linked list should have two
//attributes: val and next. val is the value of the current node, and next is a pointer/
//reference to the next node. If you want to use the doubly linked list, you will
//need one more attribute prev to indicate the previous node in the linked list.
//Assume all nodes in the linked list are 0-indexed.
//
// Implement the MyLinkedList class:
//
//
// MyLinkedList() Initializes the MyLinkedList object.
// int get(int index) Get the value of the indexᵗʰ node in the linked list. If
//the index is invalid, return -1.
// void addAtHead(int val) Add a node of value val before the first element of
//the linked list. After the insertion, the new node will be the first node of the
//linked list.
// void addAtTail(int val) Append a node of value val as the last element of
//the linked list.
// void addAtIndex(int index, int val) Add a node of value val before the indexᵗ
//ʰ node in the linked list. If index equals the length of the linked list, the
//node will be appended to the end of the linked list. If index is greater than the
//length, the node will not be inserted.
// void deleteAtIndex(int index) Delete the indexᵗʰ node in the linked list, if
//the index is valid.
//
//
//
// Example 1:
//
//
//Input
//["MyLinkedList", "addAtHead", "addAtTail", "addAtIndex", "get",
//"deleteAtIndex", "get"]
//[[], [1], [3], [1, 2], [1], [1], [1]]
//Output
//[null, null, null, null, 2, null, 3]
//
//Explanation
//MyLinkedList myLinkedList = new MyLinkedList();
//myLinkedList.addAtHead(1);
//myLinkedList.addAtTail(3);
//myLinkedList.addAtIndex(1, 2);    // linked list becomes 1->2->3
//myLinkedList.get(1);              // return 2
//myLinkedList.deleteAtIndex(1);    // now the linked list is 1->3
//myLinkedList.get(1);              // return 3
//
//
//
// Constraints:
//
//
// 0 <= index, val <= 1000
// Please do not use the built-in LinkedList library.
// At most 2000 calls will be made to get, addAtHead, addAtTail, addAtIndex and
//deleteAtIndex.
//
//
// Related Topics Linked List Design 👍 2448 👎 1533

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::rc::Rc;

type NLink = Option<Rc<RefCell<Node>>>;

struct Node {
    elem: i32,
    prev: NLink,
    next: NLink,
}

impl Node {
    fn new(elem: i32, prev: NLink, next: NLink) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { elem, prev, next }))
    }
}

pub struct MyLinkedList {
    len: usize,
    head: NLink,
    tail: NLink,
}

impl MyLinkedList {
    pub fn new() -> Self {
        MyLinkedList {
            len: 0,
            head: None,
            tail: None,
        }
    }

    pub fn get(&self, index: i32) -> i32 {
        if self.len == 0 || index < 0 || index >= self.len as i32 {
            return -1;
        }

        let mut p = self.head.clone();
        for _ in 0..index {
            let next = p.unwrap().borrow().next.clone();
            p = next;
        }

        p.map(|curr| curr.borrow().elem).unwrap_or(-1)
    }

    pub fn add_at_head(&mut self, val: i32) {
        let new_node = Node::new(val, None, None);
        match self.head.take() {
            None => {
                // list is empty
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            }
        }
        self.len += 1;
    }

    pub fn add_at_tail(&mut self, val: i32) {
        let new_node = Node::new(val, None, None);
        match self.tail.take() {
            None => {
                // list is empty
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_node);
            }
        }
        self.len += 1;
    }

    pub fn add_at_index(&mut self, index: i32, val: i32) {
        if index < 0 || index > self.len as i32 {
            return;
        }
        match index as usize {
            0 => {
                // insert at head
                self.add_at_head(val);
            }
            idx if idx == self.len => {
                // insert at tail
                self.add_at_tail(val);
            }
            _ => {
                // insert at middle
                let new_node = Node::new(val, None, None);
                let mut prev = self.head.clone();
                // Move p to the previous element of the element to be deleted
                for _ in 0..index - 1 {
                    let node = prev.unwrap().borrow().next.clone();
                    prev = node;
                }

                prev.map(|prev| {
                    if let Some(next) = prev.borrow_mut().next.take() {
                        next.borrow_mut().prev = Some(new_node.clone());
                        new_node.borrow_mut().next = Some(next);
                    }
                    new_node.borrow_mut().prev = Some(prev.clone());
                    prev.borrow_mut().next = Some(new_node);
                });
                self.len += 1;
            }
        }
    }

    pub fn delete_at_head(&mut self) -> i32 {
        self.head
            .take()
            .map(|old_head| {
                match old_head.borrow_mut().next.take() {
                    None => {
                        self.tail = None;
                    }
                    Some(next) => {
                        next.borrow_mut().prev = None;
                        self.head = Some(next);
                    }
                }
                self.len -= 1;
                old_head.borrow().elem
            })
            .unwrap_or(-1)
    }

    pub fn delete_at_tail(&mut self) -> i32 {
        self.tail
            .take()
            .map(|old_tail| {
                match old_tail.borrow_mut().prev.take() {
                    None => {
                        self.head = None;
                    }
                    Some(prev) => {
                        prev.borrow_mut().next = None;
                        self.tail = Some(prev);
                    }
                }
                self.len -= 1;
                old_tail.borrow().elem
            })
            .unwrap_or(-1)
    }

    pub fn delete_at_index(&mut self, index: i32) -> i32 {
        if self.len == 0 || index < 0 || index >= self.len as i32 {
            return -1;
        }

        match index as usize {
            0 => self.delete_at_head(),
            idx if idx == (self.len - 1) => self.delete_at_tail(),
            _ => {
                let mut del = self.head.clone();
                // Move p to the element to be deleted
                for _ in 0..index {
                    let node = del.unwrap().borrow().next.clone();
                    del = node;
                }

                del.map(|del| {
                    let mut del = del.borrow_mut();
                    del.prev.take().map(|prev| {
                        del.next.take().map(|next| {
                            next.borrow_mut().prev = Some(prev.clone());
                            prev.borrow_mut().next = Some(next);
                        });
                    });
                    self.len -= 1;
                    del.elem
                })
                .unwrap_or(-1)
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
