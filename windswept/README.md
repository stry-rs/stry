# windswept

A JSX like template engine.

## Examples

```rust
// NOTE: windswept muts be imported like this
use windswept::{Render, rsx};

// Component functions return `impl Render`
// `impl Render + 'lifetime` if there are lifetimes required
fn button(text: &str) -> impl Render + '_ {
    rsx! {
        <>
            <div>{text}</div>
        </>
    }
}

fn main() {
    let fragment = rsx! {
        <>
            {button("Hello World!")}
        </>
    }
    .render()
    .expect("Fragment failed to render");

    assert_eq!("<div>Hello World!</div>", fragment.as_str());
}
```
