---
okf_version: "0.2"
type: Function
title: power_label
resource: crates/oxide-erc/src/rules/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/rules/tests/power_label
language: rust
---

# power_label

## Signature

```rust
fn power_label(text: &str, pos: Point) -> ErcLabel
```

## Source
Lines 19–26 in `crates/oxide-erc/src/rules/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-erc/src/rules/tests.md) |
| called_by | [missing_power_flag_is_independent_of_label_order](/crates/oxide-erc/src/rules/tests/missing_power_flag_is_independent_of_label_order.md) |
