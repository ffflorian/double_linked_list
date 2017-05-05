#[derive(Clone)]
pub struct ListElement {
    value: String,
    next: Option<Box<ListElement>>,
    prev: Option<Box<ListElement>>
}

impl ListElement {
  pub fn new(value: String) -> ListElement {
    ListElement {
      value: value,
      next: None,
      prev: None
    }
  }

  pub fn new_from_existing(value: String, prev: Option<Box<ListElement>>, next: Option<Box<ListElement>>) -> ListElement {
    ListElement {
      value: value,
      next: next,
      prev: prev
    }
  }

  pub fn get_next(&self) -> Option<Box<ListElement>> {
    self.next.to_owned()
  }

  pub fn set_next(&mut self, element: ListElement) {
    self.next = Some(Box::new(element));
  }

  pub fn get_prev(&self) -> Option<Box<ListElement>> {
    self.prev.to_owned()
  }

  pub fn set_prev(&mut self, element: ListElement) {
    self.prev = Some(Box::new(element));
  }

  pub fn get_value(&self) -> String {
    self.value.to_owned()
  }

  pub fn set_value(&mut self, value: String) {
    self.value = value;
  }
}
