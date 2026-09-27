---
okf_version: "0.2"
type: Function
title: apply_symbol_parts
resource: crates/oxide-app/src/library/editor/symbol/updates/parts.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/parts/apply_symbol_parts
language: rust
---

# apply_symbol_parts

## Signature

```rust
pub(super) fn apply_symbol_parts(editor: &mut SymEditor, msg: SymbolEditorMsg)
```

## Visibility

- `pub(super)`

## Source
Lines 6–85 in `crates/oxide-app/src/library/editor/symbol/updates/parts.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parts](/crates/oxide-app/src/library/editor/symbol/updates/parts.md) |
| calls | [commit_or_discard_polygon](/crates/oxide-app/src/library/editor/symbol/updates/mod/commit_or_discard_polygon.md) |
| calls | [close_pickers](/crates/oxide-app/src/library/editor/symbol/updates/mod/close_pickers.md) |
| calls | [max_part_number](/crates/oxide-app/src/library/editor/symbol/state/mod/max_part_number.md) |
| calls | [push_undo](/crates/oxide-app/src/library/editor/symbol/updates/mod/push_undo.md) |
| calls | [delete_unit](/crates/oxide-app/src/library/editor/symbol/state/mod/delete_unit.md) |
| called_by | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
| called_by | [new_part_flushes_a_staged_polygon_onto_the_part_it_was_drawn_on](/crates/oxide-app/src/library/editor/symbol/updates/parts/new_part_flushes_a_staged_polygon_onto_the_part_it_was_drawn_on.md) |
| called_by | [next_part_discards_a_short_staged_polygon](/crates/oxide-app/src/library/editor/symbol/updates/parts/next_part_discards_a_short_staged_polygon.md) |
| called_by | [next_part_flushes_a_staged_polygon_onto_the_part_it_was_drawn_on](/crates/oxide-app/src/library/editor/symbol/updates/parts/next_part_flushes_a_staged_polygon_onto_the_part_it_was_drawn_on.md) |
