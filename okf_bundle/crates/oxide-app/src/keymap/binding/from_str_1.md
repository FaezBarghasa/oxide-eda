---
okf_version: "0.2"
type: Function
title: from_str
resource: crates/oxide-app/src/keymap/binding.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/keymap/binding/from_str_1
language: rust
---

# from_str

## Signature

```rust
fn from_str(source: &str) -> Result<Self, Self::Err>
```

## Source
Lines 208–232 in `crates/oxide-app/src/keymap/binding.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [binding](/crates/oxide-app/src/keymap/binding.md) |
| calls | [parse_key_token](/crates/oxide-app/src/keymap/binding/parse_key_token.md) |
