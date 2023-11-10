# `biome_deserialize`

`biome_deserialize` is a framework for deserializing Rust data structures generically.

The crate consists of data structures that know how to deserialize themselves along with data formats that know how to deserialize data.
It provides the layer by which these two groups interact with each other,
allowing any supported data structure to be deserialized using any supported data format.

`biome_deserialize` is designed for textual data formats.
It assumes that every supported data formats supports the following types:

- null-like values;
- boolean;
- number -- integers and floats;
- string;
- array;
- maps of key-value pairs (covers objects).

This crate is inspired by [serde](https://serde.rs/).
The only supported data format is JSON.

## Design overview

The crate provides three traits:

- `Deserializable`;
- `DeserializableValue`;
- `DeserializationVisitor`.

A data structure that knows how to deserialize itself is one that implements the `Deserializable` trait.

`DeserializableValue` is implemented by data formats such as _JSON_.

Simple implementations of `Deserializable` can reuse other deserializable data structures.
For instance, an enumeration that corresponds to a string among A, B, and C, can first deserialize a string and then check that the string is one of its values.

Data structures that cannot directly use another deserializable data structures, use a visitor.
A visitor is generally a zero-sized data structure that implements the `DeserializationVisitor` trait.
A [visitor](https://en.wikipedia.org/wiki/Visitor_pattern) is a well-known design pattern.
It allows selecting an implementation based on the deserialized type without bothering of data format details.

## Usage examples

### Deserializing common types

`biome_deserialize` implements `Deserializable` for common Rust data structure.

In the following example, we deserialize a boolean, an array of integers, and an unordered map of string-integer pairs.

```rust
use biome_deserialize::json::deserialize_from_json_str;
use biome_deserialize::Deserialized;
use biome_json_parser::JsonParserOptions;

let json = "false";
let Deserialized {
    deserialized,
    diagnostics,
} = deserialize_from_json_str::<bool>(&source, JsonParserOptions::default());
assert_eq!(deserialized, Some(false));

let json = "[0, 1]";
let Deserialized {
    deserialized,
    diagnostics,
} = deserialize_from_json_str::<Vec<u8>>(&source, JsonParserOptions::default());
assert_eq!(deserialized, Some(vec![0, 1]));

use std::collections::HashMap;
let json = r#"{ "a": 0, "b": 1 }"#;
let Deserialized {
    deserialized,
    diagnostics,
} = deserialize_from_json_str::<HashMap<String, u8>>(&source, JsonParserOptions::default());
assert_eq!(deserialized, Some(HashMap::from([("a".to_string(), 0), ("b".to_string(), 1)])));
```

### Custom integer range

...WIP...

Sometimes you want to deserialize an integer and ensure that it is between two given integers.

For instance, let's assume we want to deserialize a percentage represented by an integer between 0 and 100.
We can use the new-type pattern in Rust:

```rust
pub struct Percentage(u8);
```
