---
okf_version: "0.2"
type: Function
title: catalog_id_with_no_bridge_arm_is_a_no_op_and_does_not_panic
description: "`clear_net_highlighting` is a real catalog entry"
resource: crates/oxide-app/src/app/command/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/command/mod/catalog_id_with_no_bridge_arm_is_a_no_op_and_does_not_panic
language: rust
---

# catalog_id_with_no_bridge_arm_is_a_no_op_and_does_not_panic

`clear_net_highlighting` is a real catalog entry

## Signature

```rust
fn catalog_id_with_no_bridge_arm_is_a_no_op_and_does_not_panic()
```

## Decorators

- `test`

## Docstring

`clear_net_highlighting` is a real catalog entry
(`keymap/catalog/schematic.rs`) with no arm in `core_to_message`
yet — it must resolve to a no-op too, not panic.
[test]

## Source
Lines 86–98 in `crates/oxide-app/src/app/command/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [command](/crates/oxide-app/src/app/command/mod.md) |
