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
use std::ptr::null_mut;

type NLink = *mut Node;

struct Node {
    elem: i32,
    prev: NLink,
    next: NLink,
}

impl Node {
    fn new(elem: i32, prev: NLink, next: NLink) -> *mut Node {
        Box::into_raw(Box::new(Node { elem, prev, next }))
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
            head: null_mut(),
            tail: null_mut(),
        }
    }

    pub fn get(&self, index: i32) -> i32 {
        if self.len == 0 || index < 0 || index >= self.len as i32 {
            return -1;
        }

        let mut p = self.head;
        for _ in 0..index {
            unsafe {
                p = (*p).next;
            }
        }

        unsafe { (*p).elem }
    }

    pub fn add_at_head(&mut self, val: i32) {
        let new_node = Node::new(val, null_mut(), null_mut());
        if self.len == 0 {
            self.tail = new_node;
            self.head = new_node;
        } else {
            unsafe {
                (*self.head).prev = new_node;
                (*new_node).next = self.head;
            }
            self.head = new_node;
        }
        self.len += 1;
    }

    pub fn add_at_tail(&mut self, val: i32) {
        let new_node = Node::new(val, null_mut(), null_mut());
        if self.len == 0 {
            self.head = new_node;
            self.tail = new_node;
        } else {
            unsafe {
                (*new_node).prev = self.tail;
                (*self.tail).next = new_node;
            }
            self.tail = new_node;
        }
        self.len += 1;
    }

    pub fn add_at_index(&mut self, index: i32, val: i32) {
        if index < 0 || index > self.len as i32 {
            return;
        }
        match index as usize {
            0 => self.add_at_head(val),
            idx if idx == self.len => self.add_at_tail(val),
            _ => {
                let new_node = Node::new(val, null_mut(), null_mut());
                let mut prev = self.head;
                // Move p to the previous element of the element to be deleted
                for _ in 0..index - 1 {
                    unsafe {
                        prev = (*prev).next;
                    }
                }
                unsafe {
                    let next = (*prev).next;
                    (*next).prev = new_node;
                    (*new_node).next = next;
                    (*new_node).prev = prev;
                    (*prev).next = new_node;
                }
                self.len += 1;
            }
        }
    }

    pub fn delete_at_head(&mut self) -> i32 {
        if self.len == 0 {
            return -1;
        }
        let head = self.head;
        self.len -= 1;
        if self.len == 0 {
            self.tail = null_mut();
        }
        unsafe {
            self.head = (*self.head).next;
            Box::from_raw(head).elem
        }
    }

    pub fn delete_at_tail(&mut self) -> i32 {
        if self.len == 0 {
            return -1;
        }
        let tail = self.tail;
        self.len -= 1;
        if self.len == 0 {
            self.head = null_mut();
        }
        unsafe {
            self.tail = (*self.tail).prev;
            Box::from_raw(tail).elem
        }
    }

    pub fn delete_at_index(&mut self, index: i32) -> i32 {
        if self.len == 0 || index < 0 || index >= self.len as i32 {
            return -1;
        }

        match index as usize {
            0 => self.delete_at_head(),
            idx if idx == (self.len - 1) => self.delete_at_tail(),
            _ => {
                let mut del = self.head;
                // Move p to the element to be deleted
                for _ in 0..index {
                    unsafe {
                        del = (*del).next;
                    }
                }

                unsafe {
                    let prev = (*del).prev;
                    let next = (*del).next;
                    (*next).prev = prev;
                    (*prev).next = next;
                    self.len -= 1;
                    Box::from_raw(del).elem
                }
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
