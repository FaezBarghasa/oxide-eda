---
okf_version: "0.2"
type: Function
title: delete_selected
description: "Delete whatever is currently selected. Returns `Some(new_sel)` if"
resource: crates/oxide-app/src/library/editor/symbol/state/hit_test.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/hit_test/delete_selected
language: rust
---

# delete_selected

Delete whatever is currently selected. Returns `Some(new_sel)` if

## Signature

```rust
pub fn delete_selected(
    sym: &mut Symbol,
    sel: Option<SymbolSelection>,
) -> Option<Option<SymbolSelection>>
```

## Visibility

- `pub`

## Docstring

Delete whatever is currently selected. Returns `Some(new_sel)` if
the caller should update its selection (typically `None` after a
pin removal), or `None` if no selection change is needed.

## Source
Lines 8–55 in `crates/oxide-app/src/library/editor/symbol/state/hit_test.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hit_test](/crates/oxide-app/src/library/editor/symbol/state/hit_test.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
