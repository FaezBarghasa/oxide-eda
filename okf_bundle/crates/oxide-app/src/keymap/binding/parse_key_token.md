---
okf_version: "0.2"
type: Function
title: parse_key_token
resource: crates/oxide-app/src/keymap/binding.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/keymap/binding/parse_key_token
language: rust
---

# parse_key_token

## Signature

```rust
fn parse_key_token(part: &str) -> Result<KeyToken, KeyParseError>
```

## Source
Lines 235–266 in `crates/oxide-app/src/keymap/binding.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [binding](/crates/oxide-app/src/keymap/binding.md) |
| called_by | [from_str](/crates/oxide-app/src/keymap/binding/from_str.md) |
