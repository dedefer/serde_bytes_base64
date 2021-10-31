/*!
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
*/

use std::ops::Deref;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Bytes {
    pub val: Vec<u8>,
}

impl Deref for Bytes {
    type Target = [u8];
    fn deref(&self) -> &Self::Target { &self.val }
}

impl From<Vec<u8>> for Bytes {
    fn from(val: Vec<u8>) -> Self { Bytes { val } }
}

impl From<Bytes> for Vec<u8> {
    fn from(val: Bytes) -> Self { val.val }
}

impl Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let encoded = base64::encode(&self.val);
        serializer.serialize_str(&encoded)
    }
}

impl<'de> Deserialize<'de> for Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let encoded = <&str>::deserialize(deserializer)?;
        base64::decode(encoded)
            .map_err(serde::de::Error::custom)
            .map(Bytes::from)
    }
}

#[cfg(test)]
mod tests {
    use serde::{Serialize, Deserialize};
    use crate::Bytes;

    #[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
    struct Val { val: Bytes }

    #[test]
    fn test_serialize() {
        let result = serde_json::to_string(&Val { val: vec![1, 2 ,3].into() }).unwrap();
        assert_eq!(result, r#"{"val":"AQID"}"#.to_owned());
    }

    #[test]
    fn test_deserialize() {
        let result: Val = serde_json::from_str(r#"{"val":"AQID"}"#).unwrap();
        assert_eq!(result, Val { val: vec![1, 2 ,3].into() });
    }
}
