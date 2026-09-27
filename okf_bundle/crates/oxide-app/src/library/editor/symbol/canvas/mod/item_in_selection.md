---
okf_version: "0.2"
type: Function
title: item_in_selection
description: "Returns `true` when `item` (a single-element selection) belongs to the"
resource: crates/oxide-app/src/library/editor/symbol/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/mod/item_in_selection
language: rust
---

# item_in_selection

Returns `true` when `item` (a single-element selection) belongs to the

## Signature

```rust
fn item_in_selection(group: &SymbolSelection, item: &SymbolSelection) -> bool
```

## Docstring

Returns `true` when `item` (a single-element selection) belongs to the
multi-element `group` selection. Used to decide whether a click on an
already-selected item should start a group drag rather than replace the
selection.

## Source
Lines 330–344 in `crates/oxide-app/src/library/editor/symbol/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/library/editor/symbol/canvas/mod.md) |
| called_by | [on_left_press](/crates/oxide-app/src/library/editor/symbol/canvas/input/tools/on_left_press.md) |
