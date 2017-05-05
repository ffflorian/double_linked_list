use list_element::ListElement;

#[derive(Clone)]
pub struct LinkedList {
    head: Option<Box<ListElement>>,
    tail: Option<Box<ListElement>>,
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

  pub fn get_head(&self) -> Option<Box<ListElement>> {
    self.head.to_owned()
  }

  pub fn set_head(&mut self, head: ListElement) {
    self.head = Some(Box::new(head));
  }

  pub fn get_tail(&self) -> Option<Box<ListElement>> {
    self.tail.to_owned()
  }

  pub fn set_tail(&mut self, tail: ListElement) {
    self.tail = Some(Box::new(tail));
  }
}
