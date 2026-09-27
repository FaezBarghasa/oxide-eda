---
okf_version: "0.2"
type: Function
title: contains_selected_sheet_item
description: "`ChildSheet` / `SheetPin` share the same nested-ownership shape in"
resource: crates/oxide-engine/src/transform/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/transform/mod/contains_selected_sheet_item_1
language: rust
---

# contains_selected_sheet_item

`ChildSheet` / `SheetPin` share the same nested-ownership shape in

## Signature

```rust
fn contains_selected_sheet_item(&self, item: &SelectedItem) -> Option<bool>
```

## Docstring

`ChildSheet` / `SheetPin` share the same nested-ownership shape in
both `contains_selected_item` and `remove_selected_item`, which
pushed both matches past the house ~50-line cap. Split out so
each caller stays a flat, single-purpose match. `None` means
"not a sheet-owned kind — fall through to the caller's match".

## Source
Lines 66–82 in `crates/oxide-engine/src/transform/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-engine/src/transform/mod.md) |
