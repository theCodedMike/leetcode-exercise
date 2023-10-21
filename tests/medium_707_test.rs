use leetcode_exercise::leetcode::editor::en::_707_design_linked_list::MyLinkedList;

#[test]
fn design_linked_list_1() {
    let mut list = MyLinkedList::new();
    list.add_at_head(1);
    list.add_at_tail(3);
    list.add_at_index(1, 2);
    let val = list.get(1);
    assert_eq!(val, 2);
    list.delete_at_index(1);
    let val = list.get(1);
    assert_eq!(val, 3);
}

#[test]
fn design_linked_list_2() {
    let mut list = MyLinkedList::new();
    list.add_at_head(1);
    let val = list.delete_at_index(0);
    assert_eq!(val, 1);
}

#[test]
fn design_linked_list_3() {
    let mut list = MyLinkedList::new();
    list.add_at_head(1);
    list.add_at_tail(3);
    list.add_at_index(1, 2);
    let val = list.get(1);
    assert_eq!(val, 2);
    list.delete_at_index(0);
    let val = list.get(0);
    assert_eq!(val, 2);
}
