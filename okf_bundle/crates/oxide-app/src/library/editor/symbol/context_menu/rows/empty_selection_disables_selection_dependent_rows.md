---
okf_version: "0.2"
type: Function
title: empty_selection_disables_selection_dependent_rows
description: "Empty selection: Join into Polygon, Delete, and Deselect All"
resource: crates/oxide-app/src/library/editor/symbol/context_menu/rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/context_menu/rows/empty_selection_disables_selection_dependent_rows
language: rust
---

# empty_selection_disables_selection_dependent_rows

Empty selection: Join into Polygon, Delete, and Deselect All

## Signature

```rust
fn empty_selection_disables_selection_dependent_rows()
```

## Decorators

- `test`

## Docstring

Empty selection: Join into Polygon, Delete, and Deselect All
are disabled; Select All and Fit to Window stay enabled.
[test]

## Source
Lines 186–196 in `crates/oxide-app/src/library/editor/symbol/context_menu/rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rows](/crates/oxide-app/src/library/editor/symbol/context_menu/rows.md) |
| calls | [build_symbol_context_menu_rows](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/build_symbol_context_menu_rows.md) |
