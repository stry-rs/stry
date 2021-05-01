# (stry) remark

Remark is a HTML to Markdown converter and soon&trade; to be formatter.

## Examples

```rust
use stry_remark::convert;

fn main() {
    let input = "<h1>Hello</h1>";
    let output = "# Hello";

    assert_eq!(
        output,
        convert(input).expect("Unable to convert markdown"),
    );
}
```
