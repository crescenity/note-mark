# note-mark

[![crates.io](https://img.shields.io/crates/v/note-mark.svg)](https://crates.io/crates/note-mark)
[![docs.rs](https://docs.rs/note-mark/badge.svg)](https://docs.rs/note-mark)

A markdown parser under development.

Please read the [documentation](https://docs.rs/note-mark/).

**Note: This is still a work in progress. Do not use it.**

## Example

```toml
[dependencies]
note_mark = "0.0.2"
```

```rust
use note_mark::prelude::*;

fn main() {
    let markdown = Markdown::default();

    let html = markdown.execute("# Hello, world!\n\nThis is a new line.");

    assert_eq!(html, "<h1>Hello, world!</h1><p>This is a new line.</p>");
}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
