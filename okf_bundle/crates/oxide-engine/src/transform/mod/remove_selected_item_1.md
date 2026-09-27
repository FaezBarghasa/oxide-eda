---
okf_version: "0.2"
type: Function
title: remove_selected_item
resource: crates/oxide-engine/src/transform/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/transform/mod/remove_selected_item_1
language: rust
---

# remove_selected_item

## Signature

```rust
pub(super) fn remove_selected_item(&mut self, item: &SelectedItem) -> bool
```

## Visibility

- `pub(super)`

## Source
Lines 84–127 in `crates/oxide-engine/src/transform/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-engine/src/transform/mod.md) |
| calls | [remove_by_uuid](/crates/oxide-engine/src/transform/mod/remove_by_uuid.md) |
| calls | [drawing_uuid](/crates/oxide-engine/src/transform/mod/drawing_uuid.md) |
