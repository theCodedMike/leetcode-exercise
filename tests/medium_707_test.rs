use leetcode_exercise::leetcode::editor::en::_707_design_linked_list::MyLinkedList;

#[test]
fn design_linked_list() {
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
