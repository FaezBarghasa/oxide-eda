---
okf_version: "0.2"
type: Function
title: rotate_selected_item
resource: crates/oxide-engine/src/transform/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/transform/mod/rotate_selected_item_1
language: rust
---

# rotate_selected_item

## Signature

```rust
pub(super) fn rotate_selected_item(&mut self, item: &SelectedItem, angle_degrees: f64) -> bool
```

## Visibility

- `pub(super)`

## Source
Lines 417–472 in `crates/oxide-engine/src/transform/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-engine/src/transform/mod.md) |
| calls | [normalize_degrees](/crates/oxide-engine/src/transform/mod/normalize_degrees.md) |
| calls | [autoplace_fields](/crates/oxide-engine/src/transform/autoplace/autoplace_fields.md) |
