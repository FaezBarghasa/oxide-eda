---
okf_version: "0.2"
type: Function
title: net_label
resource: crates/oxide-erc/src/rules/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-erc"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-erc/src/rules/tests/net_label
language: rust
---

# net_label

## Signature

```rust
fn net_label(text: &str, pos: Point) -> ErcLabel
```

## Source
Lines 40–47 in `crates/oxide-erc/src/rules/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-erc/src/rules/tests.md) |
| called_by | [missing_power_flag_is_independent_of_label_order](/crates/oxide-erc/src/rules/tests/missing_power_flag_is_independent_of_label_order.md) |
| called_by | [net_label_conflict_is_independent_of_label_order](/crates/oxide-erc/src/rules/tests/net_label_conflict_is_independent_of_label_order.md) |
