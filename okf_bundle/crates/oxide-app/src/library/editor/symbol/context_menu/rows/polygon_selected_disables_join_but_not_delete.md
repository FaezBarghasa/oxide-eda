---
okf_version: "0.2"
type: Function
title: polygon_selected_disables_join_but_not_delete
description: "A single selected Polygon can't be joined (it's already the"
resource: crates/oxide-app/src/library/editor/symbol/context_menu/rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/context_menu/rows/polygon_selected_disables_join_but_not_delete
language: rust
---

# polygon_selected_disables_join_but_not_delete

A single selected Polygon can't be joined (it's already the

## Signature

```rust
fn polygon_selected_disables_join_but_not_delete()
```

## Decorators

- `test`

## Docstring

A single selected Polygon can't be joined (it's already the
output kind, not an eligible source) but can be deleted.
[test]

## Source
Lines 352–367 in `crates/oxide-app/src/library/editor/symbol/context_menu/rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rows](/crates/oxide-app/src/library/editor/symbol/context_menu/rows.md) |
| calls | [Graphic](/crates/oxide-types/src/schematic/mod/Graphic.md) |
| calls | [build_symbol_context_menu_rows](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/build_symbol_context_menu_rows.md) |
