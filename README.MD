## double_linked_list

![build status](https://api.travis-ci.org/ffflorian/double_linked_list.svg?branch=master)

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

use double_linked_list::LinkedList;
use double_linked_list::ListElement;

let list = LinkedList::new();
let element = ListElement::new("hello");
```

### Testing

Use `cargo` for testing:
```bash
$ cargo test
```
