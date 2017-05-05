## double_linked_list

A linked list in which every element knows about its predecessor and its successor.
```
 ______     ______     ______
| |  | |   | |  | |   | |  | |
| |  | <---> |  | <---> |  | |
|_|__|_|   |_|__|_|   |_|__|_|
  prev        n         next
```

### Usage

```rust
extern crate double_linked_list;

use linked_list::LinkedList;
use list_element::ListElement;

let list = LinkedList::new();
let element = ListElement::new("hello");
```

### Testing

Use `cargo` for testing:
```bash
$ cargo test
```
