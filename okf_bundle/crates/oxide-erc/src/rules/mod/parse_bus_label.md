---
okf_version: "0.2"
type: Function
title: parse_bus_label
resource: crates/oxide-erc/src/rules/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/rules/mod/parse_bus_label
language: rust
---

# parse_bus_label

## Signature

```rust
fn parse_bus_label(text: &str) -> Option<(&str, i64, i64)>
```

## Source
Lines 325–336 in `crates/oxide-erc/src/rules/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rules](/crates/oxide-erc/src/rules/mod.md) |
| called_by | [bus_bit_width_mismatch](/crates/oxide-erc/src/rules/mod/bus_bit_width_mismatch.md) |
