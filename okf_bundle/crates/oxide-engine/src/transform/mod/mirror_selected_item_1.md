---
okf_version: "0.2"
type: Function
title: mirror_selected_item
resource: crates/oxide-engine/src/transform/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/transform/mod/mirror_selected_item_1
language: rust
---

# mirror_selected_item

## Signature

```rust
pub(super) fn mirror_selected_item(&mut self, item: &SelectedItem, axis: MirrorAxis) -> bool
```

## Visibility

- `pub(super)`

## Source
Lines 474–497 in `crates/oxide-engine/src/transform/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-engine/src/transform/mod.md) |
| calls | [autoplace_fields](/crates/oxide-engine/src/transform/autoplace/autoplace_fields.md) |
