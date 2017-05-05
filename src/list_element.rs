#[derive(Clone)]
pub struct ListElement {
    value: String,
    next: Option<Box<ListElement>>,
    prev: Option<Box<ListElement>>
}

impl ListElement {

}
