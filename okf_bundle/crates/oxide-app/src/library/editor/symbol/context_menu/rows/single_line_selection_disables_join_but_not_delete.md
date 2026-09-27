---
okf_version: "0.2"
type: Function
title: single_line_selection_disables_join_but_not_delete
description: A single selected Line disables Join into Polygon — it can
resource: crates/oxide-app/src/library/editor/symbol/context_menu/rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/context_menu/rows/single_line_selection_disables_join_but_not_delete
language: rust
---

# single_line_selection_disables_join_but_not_delete

A single selected Line disables Join into Polygon — it can

## Signature

```rust
fn single_line_selection_disables_join_but_not_delete()
```

## Decorators

- `test`

## Docstring

A single selected Line disables Join into Polygon — it can
never close on its own — while Delete stays enabled.
[test]

## Source
Lines 250–266 in `crates/oxide-app/src/library/editor/symbol/context_menu/rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rows](/crates/oxide-app/src/library/editor/symbol/context_menu/rows.md) |
| calls | [Graphic](/crates/oxide-types/src/schematic/mod/Graphic.md) |
| calls | [build_symbol_context_menu_rows](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/build_symbol_context_menu_rows.md) |
