# side-by-side-diff

[![Latest Version](https://img.shields.io/crates/v/side-by-side-diff.svg)](https://crates.io/crates/side-by-side-diff)
[![Crates.io](https://img.shields.io/crates/d/side-by-side-diff.svg)](https://crates.io/crates/side-by-side-diff)
[![License](https://img.shields.io/github/license/tadashi-aikawa/side-by-side-diff)](https://github.com/tadashi-aikawa/side-by-side-diff/blob/main/LICENSE)

The `side-by-side-diff` crate is a wrapper of [Similar]. The `side-by-side-diff` creates side-by-side diff text from two texts.

```rust
use side_by_side_diff::create_side_by_side_diff;

fn main() {
    let diff = create_side_by_side_diff("aaa\niii\nuuu", "aaa\nii\nuuu", 20);
    println!("{diff}");
}
```

```console
$ cargo run
     1 | aaa                  |      1 | aaa                  |
     2 | iii                  |        |                      |
       |                      |      2 | ii                   |
     3 | uuu                  |      3 | uuu                  |
```

[Similar]: https://crates.io/crates/similar