---
okf_version: "0.2"
type: Function
title: splice_selection_into_polygon
description: "Composite mutation on success: one undo snapshot, remove the"
resource: crates/oxide-app/src/library/editor/symbol/updates/join.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/join/splice_selection_into_polygon
language: rust
---

# splice_selection_into_polygon

Composite mutation on success: one undo snapshot, remove the

## Signature

```rust
fn splice_selection_into_polygon(
    editor: &mut SymEditor,
    indices: &[usize],
    ring: Vec<[f64; 2]>,
    stroke_width: f64,
    part_number: u8,
)
```

## Docstring

Composite mutation on success: one undo snapshot, remove the
source graphics (descending index order), append the joined
Polygon on `part_number` (the sources' shared part — see
`state::common_graphic_part_number`, never hardcoded to the active
unit, so an all-shared source selection stays shared), and select
it. Does NOT go through `push_graphic`, which would push a second
undo snapshot.

## Source
Lines 108–133 in `crates/oxide-app/src/library/editor/symbol/updates/join.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [join](/crates/oxide-app/src/library/editor/symbol/updates/join.md) |
| calls | [push_undo](/crates/oxide-app/src/library/editor/symbol/updates/mod/push_undo.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [Graphic](/crates/oxide-types/src/schematic/mod/Graphic.md) |
| calls | [close_pickers](/crates/oxide-app/src/library/editor/symbol/updates/mod/close_pickers.md) |
| called_by | [apply_symbol_join](/crates/oxide-app/src/library/editor/symbol/updates/join/apply_symbol_join.md) |
