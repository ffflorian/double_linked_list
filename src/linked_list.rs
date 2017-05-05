use list_element::ListElement;

#[derive(Clone)]
pub struct LinkedList {
    head: Option<ListElement>,
    tail: Option<ListElement>,
    size: u32
}

impl LinkedList {
  pub fn new() -> LinkedList {
    LinkedList {
      head: None,
      tail: None,
      size: 0
    }
  }
}
