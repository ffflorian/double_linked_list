use linked_list::LinkedList;
use list_element::ListElement;

#[test]
fn create_list() {
  LinkedList::new();
}

#[test]
fn create_element() {
  ListElement::new("".to_string());
}
