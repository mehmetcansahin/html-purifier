HTML Purifier

# Example

```rust
use html_purifier::{purifier, Settings};

let settings = Settings {
    ..Settings::default()
};
let input = r#"<a href="/test" style="color: black;"><img src="/logo.png" onerror="javascript:;"/>Rust</a>"#;
let output = purifier(input, settings);
```
