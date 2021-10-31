![MIT licensed](https://img.shields.io/github/license/dedefer/serde_bytes_base64?style=for-the-badge)
[![Version](https://img.shields.io/crates/v/serde_bytes_base64?style=for-the-badge)](https://crates.io/crates/serde_bytes_base64/)
![Code Coverage](https://img.shields.io/coveralls/github/dedefer/serde_bytes_base64/main?style=for-the-badge)
![Downloads](https://img.shields.io/crates/d/serde_bytes_base64?style=for-the-badge)

# serde_bytes_base64

Wrapper for Vec<u8>, which uses base64 string as serde representation.

It implements Deserialize, Serialize and Deref to \[u8\];

[Documentation link](https://docs.rs/serde_bytes_base64/)

[Crates.io link](https://crates.io/crates/serde_bytes_base64/)

## Example

```rust
use serde::{Deserialize, Serialize};
use serde_bytes_base64::Bytes;

#[derive(Deserialize, Serialize, Debug)]
struct Val { val: Bytes }

fn main() {
  let result = serde_json::to_string(&Val { val: vec![
      133, 233, 101, 161, 106, 43,
      149, 208, 90, 177, 238, 184,
    ].into() }).unwrap();
  println!("{}", result); // {"val":"helloWorldBase64"}
}
```
