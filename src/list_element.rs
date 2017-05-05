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
}
